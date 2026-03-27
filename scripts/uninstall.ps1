[CmdletBinding()]
param(
    [string]$InstallDir = $env:AMAGI_INSTALL_DIR,
    [switch]$KeepPath,
    [switch]$KeepUserEnv,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

$BinaryName = "amagi.exe"
$BinaryBaseName = "amagi"
$HasExplicitInstallDir = -not [string]::IsNullOrWhiteSpace($InstallDir)
$PathCleaned = $false
$UserEnvCleaned = $false

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

function Expand-TemplatePath {
    param(
        [string]$PathValue
    )

    if ([string]::IsNullOrWhiteSpace($PathValue)) {
        return $null
    }

    $expanded = [regex]::Replace($PathValue, '\$[Ee]nv:([A-Za-z_][A-Za-z0-9_]*)', {
            param($match)

            $name = $match.Groups[1].Value
            $value = [Environment]::GetEnvironmentVariable($name)
            if ([string]::IsNullOrWhiteSpace($value)) {
                return $match.Value
            }

            return $value
        })

    if ($expanded.StartsWith('$HOME')) {
        $expanded = $HOME + $expanded.Substring(5)
    }

    return [Environment]::ExpandEnvironmentVariables($expanded).Trim('"')
}

function Normalize-DirectoryPath {
    param(
        [string]$PathValue
    )

    $expanded = Expand-TemplatePath -PathValue $PathValue
    if ([string]::IsNullOrWhiteSpace($expanded)) {
        return $null
    }

    try {
        return [System.IO.Path]::GetFullPath($expanded).TrimEnd('\', '/')
    }
    catch {
        return $expanded.TrimEnd('\', '/')
    }
}

function Add-UniqueDirectory {
    param(
        [System.Collections.Generic.List[string]]$Directories,
        [string]$PathValue
    )

    $normalized = Normalize-DirectoryPath -PathValue $PathValue
    if ([string]::IsNullOrWhiteSpace($normalized)) {
        return
    }

    if (-not $HasExplicitInstallDir -and (Test-SkipAutoDirectory -DirectoryPath $normalized)) {
        return
    }

    if (-not $Directories.Contains($normalized)) {
        [void]$Directories.Add($normalized)
    }
}

function Test-SkipAutoDirectory {
    param(
        [string]$DirectoryPath
    )

    $cargoBin = Normalize-DirectoryPath -PathValue (Join-Path $HOME ".cargo\bin")
    if ($cargoBin -and $DirectoryPath -ieq $cargoBin) {
        return $true
    }

    return $false
}

function Confirm-EnvironmentCleanup {
    param(
        [string]$Prompt,
        [string]$SkipLabel
    )

    if ($Force) {
        return $true
    }

    try {
        if ([Console]::IsInputRedirected) {
            Write-Host "[amagi] skipped $SkipLabel because confirmation requires an interactive terminal; rerun with -Force to allow it"
            return $false
        }
    }
    catch {
    }

    $reply = Read-Host "[amagi] $Prompt [y/N]"
    if ($reply -match '^(?i:y|yes)$') {
        return $true
    }

    Write-Host "[amagi] skipped $SkipLabel"
    return $false
}

function Get-DirectoriesFromPath {
    param(
        [string]$PathValue
    )

    $directories = [System.Collections.Generic.List[string]]::new()

    if ([string]::IsNullOrWhiteSpace($PathValue)) {
        return $directories
    }

    foreach ($entry in ($PathValue -split ';')) {
        if ([string]::IsNullOrWhiteSpace($entry)) {
            continue
        }

        $candidate = Join-Path $entry $BinaryName
        if (Test-Path $candidate -PathType Leaf) {
            Add-UniqueDirectory -Directories $directories -PathValue $entry
        }
    }

    return $directories
}

function Test-PathCleanupTargets {
    param(
        [System.Collections.Generic.List[string]]$CandidateDirs
    )

    foreach ($pathValue in @(
            [Environment]::GetEnvironmentVariable("Path", "Process"),
            [Environment]::GetEnvironmentVariable("Path", "User"),
            [Environment]::GetEnvironmentVariable("Path", "Machine")
        )) {
        foreach ($candidateDir in $CandidateDirs) {
            if (Test-PathEntry -PathValue $pathValue -Entry $candidateDir) {
                return $true
            }
        }
    }

    return $false
}

function Test-OwnedInstallDir {
    param(
        [string]$DirectoryPath
    )

    $leaf = Split-Path -Leaf $DirectoryPath
    $parent = Split-Path -Parent $DirectoryPath

    if ($leaf -eq "amagi") {
        return $true
    }

    if ($leaf -eq "bin" -and (Split-Path -Leaf $parent) -eq "amagi") {
        return $true
    }

    return $false
}

function Test-PathEntry {
    param(
        [string]$PathValue,
        [string]$Entry
    )

    if ([string]::IsNullOrWhiteSpace($PathValue) -or [string]::IsNullOrWhiteSpace($Entry)) {
        return $false
    }

    $normalizedEntry = Normalize-DirectoryPath -PathValue $Entry
    foreach ($segment in ($PathValue -split ';')) {
        $normalizedSegment = Normalize-DirectoryPath -PathValue $segment
        if ($normalizedSegment -and $normalizedSegment -ieq $normalizedEntry) {
            return $true
        }
    }

    return $false
}

function Remove-PathEntry {
    param(
        [string]$PathValue,
        [string]$Entry
    )

    if ([string]::IsNullOrWhiteSpace($PathValue) -or [string]::IsNullOrWhiteSpace($Entry)) {
        return $PathValue
    }

    $normalizedEntry = Normalize-DirectoryPath -PathValue $Entry
    $kept = foreach ($segment in ($PathValue -split ';')) {
        if ([string]::IsNullOrWhiteSpace($segment)) {
            continue
        }

        $normalizedSegment = Normalize-DirectoryPath -PathValue $segment
        if ($normalizedSegment -and $normalizedSegment -ieq $normalizedEntry) {
            continue
        }

        $segment
    }

    return ($kept -join ';')
}

function Remove-EmptyDirectoryIfOwned {
    param(
        [string]$DirectoryPath
    )

    if (-not (Test-Path $DirectoryPath -PathType Container)) {
        return
    }

    if (-not (Test-OwnedInstallDir -DirectoryPath $DirectoryPath)) {
        return
    }

    $children = Get-ChildItem -LiteralPath $DirectoryPath -Force
    if ($children.Count -gt 0) {
        return
    }

    Remove-Item -LiteralPath $DirectoryPath -Force
    Write-Host "[amagi] removed empty directory $DirectoryPath"

    $parent = Split-Path -Parent $DirectoryPath
    if ((Split-Path -Leaf $DirectoryPath) -eq "bin" -and (Split-Path -Leaf $parent) -eq "amagi") {
        if (Test-Path $parent -PathType Container) {
            $parentChildren = Get-ChildItem -LiteralPath $parent -Force
            if ($parentChildren.Count -eq 0) {
                Remove-Item -LiteralPath $parent -Force
                Write-Host "[amagi] removed empty directory $parent"
            }
        }
    }
}

function Remove-EmptyAmagiDirectory {
    param(
        [string]$DirectoryPath
    )

    if (-not (Test-Path $DirectoryPath -PathType Container)) {
        return
    }

    if ((Split-Path -Leaf $DirectoryPath) -ne "amagi") {
        return
    }

    $children = Get-ChildItem -LiteralPath $DirectoryPath -Force
    if ($children.Count -gt 0) {
        return
    }

    Remove-Item -LiteralPath $DirectoryPath -Force
    Write-Host "[amagi] removed empty directory $DirectoryPath"
}

function Remove-AmagiUserEnvEntries {
    $userEnvPath = Get-UserEnvFilePath
    if (-not (Test-Path $userEnvPath -PathType Leaf)) {
        return $false
    }

    $remainingLines = [System.Collections.Generic.List[string]]::new()
    $changed = $false

    foreach ($line in (Get-Content -LiteralPath $userEnvPath)) {
        if ($line -match '^\s*(?:export\s+)?AMAGI_[A-Z0-9_]+\s*=') {
            $changed = $true
            continue
        }

        $remainingLines.Add($line)
    }

    if (-not $changed) {
        return $false
    }

    if ($remainingLines.Count -eq 0 -or -not (($remainingLines -join [Environment]::NewLine).Trim()).Length) {
        Remove-Item -LiteralPath $userEnvPath -Force
        Write-Host "[amagi] removed empty user env file $userEnvPath"
    }
    else {
        $encoding = New-Object System.Text.UTF8Encoding -ArgumentList $false
        [System.IO.File]::WriteAllLines($userEnvPath, [string[]]$remainingLines, $encoding)
        Write-Host "[amagi] removed AMAGI_* entries from $userEnvPath"
    }

    $userEnvDir = Split-Path -Parent $userEnvPath
    Remove-EmptyAmagiDirectory -DirectoryPath $userEnvDir

    return $true
}

function Test-HasUserEnvEntries {
    $userEnvPath = Get-UserEnvFilePath
    if (-not (Test-Path $userEnvPath -PathType Leaf)) {
        return $false
    }

    foreach ($line in (Get-Content -LiteralPath $userEnvPath)) {
        if ($line -match '^\s*(?:export\s+)?AMAGI_[A-Z0-9_]+\s*=') {
            return $true
        }
    }

    return $false
}

function Get-CandidateInstallDirs {
    $directories = [System.Collections.Generic.List[string]]::new()

    Add-UniqueDirectory -Directories $directories -PathValue $InstallDir

    if (-not $HasExplicitInstallDir) {
        Add-UniqueDirectory -Directories $directories -PathValue (Get-DefaultInstallDir)

        $resolvedCommand = Get-Command $BinaryBaseName -CommandType Application -ErrorAction SilentlyContinue |
            Select-Object -First 1
        if ($resolvedCommand -and $resolvedCommand.Source) {
            Add-UniqueDirectory -Directories $directories -PathValue (Split-Path -Parent $resolvedCommand.Source)
        }

        foreach ($pathValue in @(
                [Environment]::GetEnvironmentVariable("Path", "Process"),
                [Environment]::GetEnvironmentVariable("Path", "User"),
                [Environment]::GetEnvironmentVariable("Path", "Machine")
            )) {
            foreach ($directory in (Get-DirectoriesFromPath -PathValue $pathValue)) {
                Add-UniqueDirectory -Directories $directories -PathValue $directory
            }
        }
    }

    return $directories
}

$candidateDirs = Get-CandidateInstallDirs
$removedAnyBinary = $false

foreach ($candidateDir in $candidateDirs) {
    $binaryPath = Join-Path $candidateDir $BinaryName
    if (Test-Path $binaryPath -PathType Leaf) {
        Remove-Item -LiteralPath $binaryPath -Force
        Write-Host "[amagi] removed $binaryPath"
        $removedAnyBinary = $true
    }

    Remove-EmptyDirectoryIfOwned -DirectoryPath $candidateDir
}

if (-not $KeepPath -and $candidateDirs.Count -gt 0) {
    if ((Test-PathCleanupTargets -CandidateDirs $candidateDirs) -and (Confirm-EnvironmentCleanup -Prompt "Remove matching PATH entries from user, machine, and current-session PATH?" -SkipLabel "PATH cleanup")) {
        $PathCleaned = $true

        $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
        foreach ($candidateDir in $candidateDirs) {
            $userPath = Remove-PathEntry -PathValue $userPath -Entry $candidateDir
        }
        [Environment]::SetEnvironmentVariable("Path", $userPath, "User")

        $processPath = [Environment]::GetEnvironmentVariable("Path", "Process")
        foreach ($candidateDir in $candidateDirs) {
            $processPath = Remove-PathEntry -PathValue $processPath -Entry $candidateDir
        }
        $env:Path = $processPath

        $machinePath = [Environment]::GetEnvironmentVariable("Path", "Machine")
        if (-not [string]::IsNullOrWhiteSpace($machinePath)) {
            $newMachinePath = $machinePath
            foreach ($candidateDir in $candidateDirs) {
                $newMachinePath = Remove-PathEntry -PathValue $newMachinePath -Entry $candidateDir
            }

            if ($newMachinePath -ne $machinePath) {
                try {
                    [Environment]::SetEnvironmentVariable("Path", $newMachinePath, "Machine")
                }
                catch {
                    Write-Host "[amagi] detected matching machine PATH entries but could not update them without elevation"
                }
            }
        }

        Write-Host "[amagi] removed matching PATH entries where present"
    }
}

if (-not $KeepUserEnv) {
    if ((Test-HasUserEnvEntries) -and (Confirm-EnvironmentCleanup -Prompt "Remove AMAGI_* entries from $(Get-UserEnvFilePath)?" -SkipLabel "user env cleanup")) {
        $UserEnvCleaned = Remove-AmagiUserEnvEntries
    }
}

if ($removedAnyBinary -or $PathCleaned -or $UserEnvCleaned) {
    Write-Host "[amagi] uninstall complete"
}
else {
    Write-Host "[amagi] no installed binary or persisted configuration found."
}
