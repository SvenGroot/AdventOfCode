param(
    [Parameter(Mandatory=$false)][int]$NewYear = 0,
    [Parameter(Mandatory=$false)][int]$OldYear = 0
)

if ($OldYear -eq 0) {
    $OldYear = (Get-ChildItem $PSScriptRoot -Filter "2*" | 
        Where-Object { $_.Attributes.HasFlag([System.IO.FileAttributes]::Directory) } |
        ForEach-Object { [int]$_.Name } | 
        Measure-Object -Maximum).Maximum
}

if ($NewYear -eq 0) {
    $NewYear = $OldYear + 1
}

$target = Join-Path $PSScriptRoot $NewYear
$source = Join-Path $PSScriptRoot $OldYear
if (Test-Path $target) {
    throw "Year already exists."
}

New-Item $target -ItemType Directory | Out-Null
New-Item "$target/input" -ItemType Directory | Out-Null
New-Item "$target/input/sample" -ItemType Directory | Out-Null
Copy-Item "$source/aoc" $target -Recurse
Get-Content "$source/Cargo.toml" | ForEach-Object {
    if ($_.StartsWith("version = ")) {
        "version = `"$NewYear.0.0`""

    } else {
        $_
    }
} | Set-Content "$target/Cargo.toml"

"# Advent of Code $NewYear","","[Event homepage](https://adventofcode.com/$NewYear)" | Set-Content "$target/README.md"

"Created year $NewYear, use New-Day.ps1 to create a day"
