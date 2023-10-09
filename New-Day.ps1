# Use F12 dev tools to find the session ID. In Edge, this is found under Application, then
# Storage/Cookies.
param(
    [Parameter(Mandatory=$true)][string]$SessionId,
    [Parameter(Mandatory=$false)][int]$Day = 0,
    [Parameter(Mandatory=$false)][int]$Year = 0
)

if ($Year -eq 0) {
    $Year = (Get-ChildItem $PSScriptRoot -Filter "2*" | 
        Where-Object { $_.Attributes.HasFlag([System.IO.FileAttributes]::Directory) } |
        ForEach-Object { [int]$_.Name } | 
        Measure-Object -Maximum).Maximum
}

$basePath = Join-Path $PSScriptRoot $Year
if ($Day -eq 0) {
    $Day = (Get-ChildItem $basePath -Filter "day*" | 
        ForEach-Object { $_.Name.Substring(3) } | 
        Measure-Object -Maximum).Maximum + 1
}

$source = "$PSScriptRoot/template"
$dest = "$basePath/day$($Day.ToString("00"))"
Copy-Item $source $dest -Recurse | Out-Null
Get-ChildItem $source -Recurse -File | ForEach-Object {
    $relative = [System.IO.Path]::GetRelativePath($source, $_)
    $destFile = Join-Path $dest $relative
    Write-Verbose "Doing replacements on $destFile"
    Get-Content $_ | ForEach-Object {
        $_.Replace("%DAY%", $Day).Replace("%YEAR%", $Year)
    } | Set-Content $destFile
}

$samplePath = Join-Path "$basePath" "input" "sample" "day$Day.txt"
if (Test-Path $samplePath) {
    Write-Warning "$samplePath already exists."

} else {
    New-Item "$basePath/input/sample/day$Day.txt" | Out-Null
}

$inputPath = Join-Path "$basePath" "input" "day$Day.txt"
if (Test-Path $inputPath) {
    Write-Warning "$inputPath already exists."
    
} else {
    $session = [Microsoft.PowerShell.Commands.WebRequestSession]::new()
    $cookie = [System.Net.Cookie]::new("session", $SessionId, "/", ".adventofcode.com")
    $session.Cookies.Add($cookie)
    Invoke-WebRequest "https://adventofcode.com/$Year/day/$Day/input" -WebSession $session -OutFile $inputPath 
}

"Created day $Day"
