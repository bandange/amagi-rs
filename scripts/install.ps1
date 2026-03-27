[CmdletBinding()]
param(
    [ValidateSet("Auto", "Local", "Remote")]
    [string]$Source = $(if ($env:AMAGI_INSTALL_SOURCE) { $env:AMAGI_INSTALL_SOURCE } else { "Auto" }),
    [string]$InstallDir = $env:AMAGI_INSTALL_DIR,
    [string]$Version = $(if ($env:AMAGI_INSTALL_VERSION) { $env:AMAGI_INSTALL_VERSION } else { "latest" })
)

$ErrorActionPreference = "Stop"

$BinaryName = "amagi.exe"
$RemoteRepoOwner = if ($env:AMAGI_REMOTE_REPO_OWNER) { $env:AMAGI_REMOTE_REPO_OWNER } else { "bandange" }
$RemoteRepoName = if ($env:AMAGI_REMOTE_REPO_NAME) { $env:AMAGI_REMOTE_REPO_NAME } else { "amagi-rs" }
$RemoteBaseUrl = $env:AMAGI_REMOTE_BASE_URL
$ScriptPath = if ($PSCommandPath) { $PSCommandPath } else { $MyInvocation.MyCommand.Path }
$ScriptDir = if ($ScriptPath) { Split-Path -Parent $ScriptPath } else { $null }
$RepoRoot = if ($ScriptDir) {
    try {
        (Resolve-Path (Join-Path $ScriptDir "..") -ErrorAction Stop).Path
    }
    catch {
        $null
    }
}
else {
    $null
}

function Get-DefaultInstallDir {
    return (Join-Path $env:LOCALAPPDATA "Programs\amagi\bin")
}

function Get-UserEnvFilePath {
    if (-not [string]::IsNullOrWhiteSpace($env:AMAGI_USER_ENV_FILE)) {
        return $env:AMAGI_USER_ENV_FILE
    }

    if (-not [string]::IsNullOrWhiteSpace($env:APPDATA)) {
        return (Join-Path $env:APPDATA "amagi\.env")
    }

    return (Join-Path $HOME "AppData\Roaming\amagi\.env")
}

function Get-ProjectEnvSourcePath {
    $currentDirEnv = Join-Path (Get-Location) ".env"
    if (Test-Path $currentDirEnv -PathType Leaf) {
        return (Resolve-Path $currentDirEnv).Path
    }

    if ($RepoRoot) {
        $repoEnv = Join-Path $RepoRoot ".env"
        if (Test-Path $repoEnv -PathType Leaf) {
            return (Resolve-Path $repoEnv).Path
        }
    }

    return $null
}

function Get-AmagiEnvEntries {
    param(
        [string]$Path
    )

    $entries = [ordered]@{}

    foreach ($line in (Get-Content -LiteralPath $Path)) {
        if ($line -match '^\s*(?:export\s+)?(AMAGI_[A-Z0-9_]+)\s*=') {
            $entries[$matches[1]] = $line
        }
    }

    return $entries
}

function Sync-UserEnvFile {
    param(
        [string]$SourcePath
    )

    if (-not $SourcePath) {
        Write-Host "[amagi] no project .env found in the current directory; skipped user env sync"
        return
    }

    $entries = Get-AmagiEnvEntries -Path $SourcePath
    if ($entries.Count -eq 0) {
        Write-Host "[amagi] no AMAGI_* keys found in $SourcePath; skipped user env sync"
        return
    }

    $userEnvPath = Get-UserEnvFilePath
    $userEnvDir = Split-Path -Parent $userEnvPath
    New-Item -ItemType Directory -Force -Path $userEnvDir | Out-Null

    $updatedLines = [System.Collections.Generic.List[string]]::new()
    $seenKeys = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::Ordinal)

    if (Test-Path $userEnvPath -PathType Leaf) {
        foreach ($line in (Get-Content -LiteralPath $userEnvPath)) {
            if ($line -match '^\s*(?:export\s+)?(AMAGI_[A-Z0-9_]+)\s*=') {
                $key = $matches[1]
                if ($entries.Contains($key)) {
                    $updatedLines.Add($entries[$key])
                    [void]$seenKeys.Add($key)
                }
                else {
                    $updatedLines.Add($line)
                }
            }
            else {
                $updatedLines.Add($line)
            }
        }
    }

    foreach ($key in $entries.Keys) {
        if (-not $seenKeys.Contains($key)) {
            $updatedLines.Add($entries[$key])
        }
    }

    $encoding = New-Object System.Text.UTF8Encoding -ArgumentList $false
    [System.IO.File]::WriteAllLines($userEnvPath, [string[]]$updatedLines, $encoding)

    Write-Host "[amagi] synced $($entries.Count) AMAGI_* entries to $userEnvPath"
}

function Get-LocalSourceBinary {
    $candidates = @()

    if ($ScriptDir) {
        $candidates += (Join-Path $ScriptDir $BinaryName)
    }

    if ($RepoRoot) {
        $candidates += (Join-Path $RepoRoot "target\release\$BinaryName")
        $candidates += (Join-Path $RepoRoot "target\debug\$BinaryName")
    }

    foreach ($candidate in $candidates) {
        if (Test-Path $candidate -PathType Leaf) {
            return (Resolve-Path $candidate).Path
        }
    }

    return $null
}

function Test-RepositoryWorkspace {
    return $RepoRoot -and (Test-Path (Join-Path $RepoRoot "Cargo.toml") -PathType Leaf)
}

function Build-LocalReleaseBinary {
    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if (-not $cargo) {
        return $null
    }

    if (-not $RepoRoot -or -not (Test-Path (Join-Path $RepoRoot "Cargo.toml") -PathType Leaf)) {
        return $null
    }

    Write-Host "[amagi] no local binary found, building release binary with cargo build --release"

    Push-Location $RepoRoot
    try {
        & $cargo.Source build --release
    }
    finally {
        Pop-Location
    }

    $builtBinary = Join-Path $RepoRoot "target\release\$BinaryName"
    if (Test-Path $builtBinary -PathType Leaf) {
        return (Resolve-Path $builtBinary).Path
    }

    return $null
}

function Get-RemoteAssetName {
    $arch = switch ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture) {
        ([System.Runtime.InteropServices.Architecture]::X64) { "x86_64"; break }
        ([System.Runtime.InteropServices.Architecture]::Arm64) { "aarch64"; break }
        default { throw "[amagi] unsupported architecture for remote install: $([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture)" }
    }

    return "amagi-windows-$arch.exe"
}

function Get-RemoteDownloadUrl {
    $assetName = Get-RemoteAssetName

    if (-not [string]::IsNullOrWhiteSpace($RemoteBaseUrl)) {
        return ($RemoteBaseUrl.TrimEnd("/") + "/$assetName")
    }

    if ([string]::IsNullOrWhiteSpace($RemoteRepoOwner) -or [string]::IsNullOrWhiteSpace($RemoteRepoName)) {
        throw "[amagi] remote repository configuration is empty. Set AMAGI_REMOTE_REPO_OWNER and AMAGI_REMOTE_REPO_NAME, or edit scripts/install.ps1 before using remote install."
    }

    if ($Version -eq "latest") {
        return "https://github.com/$RemoteRepoOwner/$RemoteRepoName/releases/latest/download/$assetName"
    }

    return "https://github.com/$RemoteRepoOwner/$RemoteRepoName/releases/download/$Version/$assetName"
}

function Get-RemoteBinary {
    $url = Get-RemoteDownloadUrl
    $downloadPath = Join-Path $env:TEMP ("amagi-install-" + [System.Guid]::NewGuid().ToString("N") + ".exe")

    Write-Host "[amagi] downloading $url"
    Invoke-WebRequest -Uri $url -OutFile $downloadPath
    return $downloadPath
}

function Test-PathEntry {
    param(
        [string]$PathValue,
        [string]$Entry
    )

    if ([string]::IsNullOrWhiteSpace($PathValue)) {
        return $false
    }

    foreach ($segment in ($PathValue -split ";")) {
        if ($segment.TrimEnd("\") -ieq $Entry.TrimEnd("\")) {
            return $true
        }
    }

    return $false
}

function Test-PathEntryIsFirst {
    param(
        [string]$PathValue,
        [string]$Entry
    )

    if ([string]::IsNullOrWhiteSpace($PathValue)) {
        return $false
    }

    foreach ($segment in ($PathValue -split ";")) {
        if ([string]::IsNullOrWhiteSpace($segment)) {
            continue
        }

        return $segment.TrimEnd("\") -ieq $Entry.TrimEnd("\")
    }

    return $false
}

function Set-PathEntryFirst {
    param(
        [string]$PathValue,
        [string]$Entry
    )

    if ([string]::IsNullOrWhiteSpace($Entry)) {
        return $PathValue
    }

    $updatedSegments = [System.Collections.Generic.List[string]]::new()
    $updatedSegments.Add($Entry)

    if (-not [string]::IsNullOrWhiteSpace($PathValue)) {
        foreach ($segment in ($PathValue -split ";")) {
            if ([string]::IsNullOrWhiteSpace($segment)) {
                continue
            }

            if ($segment.TrimEnd("\") -ieq $Entry.TrimEnd("\")) {
                continue
            }

            $updatedSegments.Add($segment)
        }
    }

    return ($updatedSegments -join ";")
}

function Add-InstallDirToUserPath {
    param(
        [string]$Entry
    )

    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $hadUserEntry = Test-PathEntry -PathValue $userPath -Entry $Entry
    $newUserPath = Set-PathEntryFirst -PathValue $userPath -Entry $Entry

    if ($userPath -ne $newUserPath) {
        [Environment]::SetEnvironmentVariable("Path", $newUserPath, "User")

        if ($hadUserEntry) {
            Write-Host "[amagi] moved install directory to the front of the user PATH"
        }
        else {
            Write-Host "[amagi] added install directory to the front of the user PATH"
        }
    }
    else {
        Write-Host "[amagi] install directory already has priority in the user PATH"
    }

    $processPath = $env:Path
    $hadProcessEntry = Test-PathEntry -PathValue $processPath -Entry $Entry
    $newProcessPath = Set-PathEntryFirst -PathValue $processPath -Entry $Entry

    if ($processPath -ne $newProcessPath) {
        $env:Path = $newProcessPath

        if ($hadProcessEntry) {
            Write-Host "[amagi] moved install directory to the front of PATH for the current PowerShell session"
        }
        else {
            Write-Host "[amagi] updated PATH for the current PowerShell session"
        }
    }
    elseif (Test-PathEntryIsFirst -PathValue $processPath -Entry $Entry) {
        Write-Host "[amagi] install directory already has priority in the current PowerShell session"
    }
}

function Resolve-ExecutionMode {
    switch ($Source.ToLowerInvariant()) {
        "local" { return "local" }
        "remote" { return "remote" }
        "auto" {
            $hasScriptBinary = $ScriptDir -and (Test-Path (Join-Path $ScriptDir $BinaryName) -PathType Leaf)
            $hasRepoRoot = $RepoRoot -and (Test-Path (Join-Path $RepoRoot "Cargo.toml") -PathType Leaf)
            $hasBuiltBinary = $RepoRoot -and (Test-Path (Join-Path $RepoRoot "target\release\$BinaryName") -PathType Leaf)

            if ($hasScriptBinary -or $hasRepoRoot -or $hasBuiltBinary) {
                return "local"
            }

            return "remote"
        }
        default {
            throw "[amagi] unsupported install source mode: $Source"
        }
    }
}

if ([string]::IsNullOrWhiteSpace($InstallDir)) {
    $InstallDir = Get-DefaultInstallDir
}

$InstallMode = Resolve-ExecutionMode
$SourceBinary = if ($InstallMode -eq "local") {
    $scriptBinary = if ($ScriptDir) {
        $candidate = Join-Path $ScriptDir $BinaryName
        if (Test-Path $candidate -PathType Leaf) {
            (Resolve-Path $candidate).Path
        }
    }
    else {
        $null
    }

    if ($scriptBinary) {
        $scriptBinary
    }
    elseif (Test-RepositoryWorkspace) {
        $builtBinary = Build-LocalReleaseBinary
        if ($builtBinary) {
            $builtBinary
        }
        else {
            Get-LocalSourceBinary
        }
    }
    else {
        Get-LocalSourceBinary
    }
}
else {
    Get-RemoteBinary
}

if (-not $SourceBinary) {
    if ($InstallMode -eq "local") {
        throw "[amagi] no local binary found next to the script or in target/release. Remote download is available only when this script runs in remote mode."
    }

    throw "[amagi] failed to download the remote binary."
}

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
$InstallPath = Join-Path $InstallDir $BinaryName

if ([System.IO.Path]::GetFullPath($SourceBinary) -ne [System.IO.Path]::GetFullPath($InstallPath)) {
    Copy-Item -LiteralPath $SourceBinary -Destination $InstallPath -Force
}

if ($InstallMode -eq "remote" -and (Test-Path $SourceBinary -PathType Leaf)) {
    Remove-Item -LiteralPath $SourceBinary -Force
}

Write-Host "[amagi] installed to $InstallPath"
Add-InstallDirToUserPath -Entry $InstallDir
Sync-UserEnvFile -SourcePath (Get-ProjectEnvSourcePath)
Write-Host "[amagi] restart your terminal to pick up the persisted PATH entry in new sessions"
