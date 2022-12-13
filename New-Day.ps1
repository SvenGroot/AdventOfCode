param(
    [Parameter(Mandatory=$false)][int]$Day = 0
)

if ($Day -eq 0) {
    $Day = (Get-ChildItem $PSScriptRoot -Filter "day*" | 
        ForEach-Object { $_.Name.Substring(3) } | 
        Measure-Object -Maximum).Maximum + 1
}

$source = "$PSScriptRoot/template"
$dest = "$PSScriptRoot/day$Day"
Copy-Item "$PSScriptRoot/template" $dest -Recurse | Out-Null
Get-Content "$source/Cargo.toml" | ForEach-Object {
    $_.Replace("day", "day$Day")
} | Set-Content "$dest/Cargo.toml"

New-Item "$PSScriptRoot/input/sample/day$Day.txt" | Out-Null
New-Item "$PSScriptRoot/input/day$Day.txt" | Out-Null

"Created day $Day"
