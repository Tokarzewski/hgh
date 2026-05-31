param(
  [string]$Dll = "C:\Program Files\Rhino 8\Plug-ins\Grasshopper\GH_IO.dll",
  [string]$File = "C:\Users\model\Documents\Github\hgh\gh\v8\1234 image+dividing+extrusion+speckle.ghx"
)

Add-Type -Path $Dll
$archive = New-Object GH_IO.Serialization.GH_Archive
if (-not $archive.ReadFromFile($File)) { Write-Error "Failed to read $File"; exit 1 }

function Items-Map($chunk) {
  $h = @{}
  if ($chunk) { foreach ($it in $chunk.Items) { if (-not $h.ContainsKey($it.Name)) { $h[$it.Name] = $it.InternalData } } }
  return $h
}

$root = $archive.GetRootNode
$def  = $root.FindChunk("Definition")

Write-Host "=== DOCUMENT ===" -ForegroundColor Cyan
$hdr = Items-Map ($def.FindChunk("DefinitionProperties"))
foreach ($k in "Name","Description","Date") { if ($hdr.ContainsKey($k)) { "{0,-12}: {1}" -f $k, $hdr[$k] } }

$objs  = $def.FindChunk("DefinitionObjects")
$count = $objs.GetInt32("ObjectCount")
Write-Host "`n=== OBJECTS ($count) ===" -ForegroundColor Cyan

$rows = for ($i = 0; $i -lt $count; $i++) {
  $o = $objs.FindChunk("Object", $i)
  if (-not $o) { continue }
  $oi   = Items-Map $o
  $cont = Items-Map ($o.FindChunk("Container"))
  [pscustomobject]@{
    Name         = $oi["Name"]
    NickName     = $cont["NickName"]
    TypeGuid     = $oi["GUID"]
    InstanceGuid = $cont["InstanceGuid"]
  }
}

Write-Host "`n--- Component counts by type ---" -ForegroundColor Yellow
$rows | Group-Object Name | Sort-Object Count -Descending | ForEach-Object { "{0,4}  {1}" -f $_.Count, $_.Name }

Write-Host "`n--- Full object list ---" -ForegroundColor Yellow
$rows | ForEach-Object {
  $nn = if ($_.NickName -and $_.NickName -ne $_.Name) { " [$($_.NickName)]" } else { "" }
  "{0}{1}" -f $_.Name, $nn
}

Write-Host "`nTotal objects: $($rows.Count)"
