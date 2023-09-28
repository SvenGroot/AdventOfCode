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

Get-Content "$source/src/main.rs" | ForEach-Object {
    $_.Replace("day/", "day/$Day")
} | Set-Content "$dest/src/main.rs"

New-Item "$PSScriptRoot/input/sample/day$Day.txt" | Out-Null
# I'd try to download it but it requires that you're logged in.
New-Item "$PSScriptRoot/input/day$Day.txt" | Out-Null

"Created day $Day"
