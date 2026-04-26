[CmdletBinding()]
param(
    [ValidateSet("Remote", "Local")]
    [string]$Source = $(if ($env:AMAGI_UPDATE_SOURCE) { $env:AMAGI_UPDATE_SOURCE } else { "Remote" }),
    [string]$InstallDir = $env:AMAGI_INSTALL_DIR,
    [string]$Version = $(if ($env:AMAGI_INSTALL_VERSION) { $env:AMAGI_INSTALL_VERSION } elseif ($env:AMAGI_UPDATE_VERSION) { $env:AMAGI_UPDATE_VERSION } else { "latest" }),
    [string]$InstallScriptUrl = $env:AMAGI_INSTALL_SCRIPT_URL,
    [switch]$Proxy,
    [string]$ProxyPrefix = $env:AMAGI_PROXY_PREFIX
)

$ErrorActionPreference = "Stop"

$RemoteRepoOwner = if ($env:AMAGI_REMOTE_REPO_OWNER) { $env:AMAGI_REMOTE_REPO_OWNER } else { "bandange" }
$RemoteRepoName = if ($env:AMAGI_REMOTE_REPO_NAME) { $env:AMAGI_REMOTE_REPO_NAME } else { "amagi-rs" }
$InstallScriptRef = if ($env:AMAGI_INSTALL_SCRIPT_REF) { $env:AMAGI_INSTALL_SCRIPT_REF } else { "main" }
$ScriptPath = if ($PSCommandPath) { $PSCommandPath } else { $MyInvocation.MyCommand.Path }
$ScriptDir = if ($ScriptPath) { Split-Path -Parent $ScriptPath } else { $null }

function Get-DefaultProxyPrefix {
    return "https://gh-proxy.com/"
}

function Normalize-ProxyPrefix {
    param(
        [string]$Prefix
    )

    if ([string]::IsNullOrWhiteSpace($Prefix)) {
        return ""
    }

    if ($Prefix.EndsWith("/")) {
        return $Prefix
    }

    return "$Prefix/"
}

function Add-ProxyPrefixToUrl {
    param(
        [string]$Url
    )

    if ([string]::IsNullOrWhiteSpace($ProxyPrefix)) {
        return $Url
    }

    return "$ProxyPrefix$Url"
}

$ProxyPrefix = Normalize-ProxyPrefix -Prefix $ProxyPrefix
if ($Proxy -and [string]::IsNullOrWhiteSpace($ProxyPrefix)) {
    $ProxyPrefix = Get-DefaultProxyPrefix
}

function Get-ResolvedInstallScriptUrl {
    if (-not [string]::IsNullOrWhiteSpace($InstallScriptUrl)) {
        return $InstallScriptUrl
    }

    return (Add-ProxyPrefixToUrl -Url "https://raw.githubusercontent.com/$RemoteRepoOwner/$RemoteRepoName/$InstallScriptRef/scripts/install.ps1")
}

function Invoke-LocalInstallScript {
    $localInstallScript = if ($ScriptDir) { Join-Path $ScriptDir "install.ps1" } else { $null }
    if (-not $localInstallScript -or -not (Test-Path $localInstallScript -PathType Leaf)) {
        return $false
    }

    $invokeArgs = @{
        Source = $Source
        InstallDir = $InstallDir
        Version = $Version
    }

    if ($Proxy) {
        $invokeArgs.Proxy = $true
    }

    if (-not [string]::IsNullOrWhiteSpace($ProxyPrefix)) {
        $invokeArgs.ProxyPrefix = $ProxyPrefix
    }

    Write-Host "[amagi] updating via local install script ($($Source.ToLowerInvariant()) mode)"
    & $localInstallScript @invokeArgs
    return $true
}

function Invoke-RemoteInstallScript {
    $url = Get-ResolvedInstallScriptUrl
    $tempPath = Join-Path $env:TEMP ("amagi-install-" + [System.Guid]::NewGuid().ToString("N") + ".ps1")
    $invokeArgs = @{
        Source = $Source
        InstallDir = $InstallDir
        Version = $Version
    }

    if ($Proxy) {
        $invokeArgs.Proxy = $true
    }

    if (-not [string]::IsNullOrWhiteSpace($ProxyPrefix)) {
        $invokeArgs.ProxyPrefix = $ProxyPrefix
    }

    Write-Host "[amagi] updating via $url"
    try {
        Invoke-WebRequest -Uri $url -OutFile $tempPath
        & $tempPath @invokeArgs
    }
    finally {
        if (Test-Path $tempPath -PathType Leaf) {
            Remove-Item -LiteralPath $tempPath -Force
        }
    }
}

if ($Source -eq "Local") {
    if (-not (Invoke-LocalInstallScript)) {
        throw "[amagi] local update requested but scripts/install.ps1 is not available next to update.ps1."
    }

    return
}

if (-not (Invoke-LocalInstallScript)) {
    Invoke-RemoteInstallScript
}
