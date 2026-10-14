# Rebuild the embedded catalog from individually translated terminology.
# No word-for-word machine substitution is used for missing phrases.
$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
$map = [Collections.Generic.SortedDictionary[string,string]]::new([StringComparer]::Ordinal)
foreach ($line in Get-Content (Join-Path $root 'tibetan-terms.txt') -Encoding utf8) {
    if (-not $line -or $line.StartsWith('#')) { continue }
    $parts = $line.Split('|', 2)
    if ($parts.Count -ne 2 -or -not $parts[1]) { throw "Invalid term: $line" }
    $map[$parts[0]] = $parts[1]
}
$entries = [Collections.Generic.SortedDictionary[string,string]]::new([StringComparer]::Ordinal)
foreach ($key in $map.Keys) {
    foreach ($suffix in @('', '…', ':')) { $entries[$key + $suffix] = $map[$key] + $suffix }
    if ($key.EndsWith(' Tool')) {
        $short = $key.Substring(0, $key.Length - 5)
        if (-not $map.ContainsKey($short)) { $entries[$short] = $map[$key] }
    }
}
# Standard names and numeric ratios keep their established spelling.
$catalog = Get-Content (Join-Path $root 'crates/ui-egui/src/menu_catalog.rs') -Raw
foreach ($match in [regex]::Matches($catalog, '"([^"\r\n]*)"')) {
    $key = $match.Groups[1].Value
    if ($key -match '^(D1/|DVCPRO|HDV|Anamorphic|Windows LCD$|HSB/HSL$)') { $entries[$key] = $key }
}
$lines = [Collections.Generic.List[string]]::new()
$lines.Add('# Tibetan (bo). Dictionary-guided original translations. See docs/localization-bo.md.')
foreach ($key in $entries.Keys) { $lines.Add("`t$key`t$($entries[$key])") }
foreach ($line in Get-Content (Join-Path $root 'tibetan-context-terms.txt') -Encoding utf8) {
    if (-not $line -or $line.StartsWith('#')) { continue }
    $parts = $line.Split('|', 3)
    if ($parts.Count -ne 3 -or -not $parts[2]) { throw "Invalid contextual term: $line" }
    $lines.Add(($parts -join "`t"))
}
[IO.File]::WriteAllLines((Join-Path $root 'crates/ui-egui/src/i18n/bo.tsv'), $lines, [Text.UTF8Encoding]::new($false))
Write-Output "Wrote $($entries.get_Count()) Tibetan catalog rows."
