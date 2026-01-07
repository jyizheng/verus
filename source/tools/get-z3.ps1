# Script to get z3 from the local z3 directory in the repository
# The z3 directory contains Z3 version 4.12.5 x64-glibc-2.31

# Get the directory where this script is located
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
# Repository root is two levels up from source/tools
$RepoRoot = (Resolve-Path "$ScriptDir\..\..")
$Z3LocalDir = Join-Path $RepoRoot "z3"

# Delete existing z3.exe if present
if (Test-Path "z3.exe") {
    Remove-Item "z3.exe" -Force
}

# Check if local z3 directory exists
if (Test-Path $Z3LocalDir) {
    $z3Binary = Join-Path $Z3LocalDir "bin\z3.exe"
    if (Test-Path $z3Binary) {
        Copy-Item $z3Binary -Destination "."
        Write-Host "z3.exe copied from local directory: $Z3LocalDir"
        Write-Host "z3.exe located at $(Get-Location)\z3.exe"
    } else {
        Write-Error "Error: z3.exe binary not found in $Z3LocalDir\bin\"
        exit 1
    }
} else {
    Write-Error "Error: Local z3 directory not found at $Z3LocalDir"
    exit 1
}
