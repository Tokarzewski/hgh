param(
  [string]$Dll  = "C:\Program Files\Rhino 8\Plug-ins\Grasshopper\GH_IO.dll",
  [string]$File = "C:\Users\model\Documents\Github\hgh\gh\v8\1234 image+dividing+extrusion+speckle.ghx"
)

Add-Type -Path $Dll
$archive = New-Object GH_IO.Serialization.GH_Archive
if (-not $archive.ReadFromFile($File)) { throw "read failed" }
$objs = $archive.GetRootNode.FindChunk("Definition").FindChunk("DefinitionObjects")
$count = $objs.GetInt32("ObjectCount")

function Val($c,$name){ ($c.Items | Where-Object { $_.Name -eq $name } | Select-Object -First 1).InternalData }
function Sources($c){ $c.Items | Where-Object { $_.Name -eq "Source" } | ForEach-Object { [string]$_.InternalData } }

# recursively collect every InstanceGuid item value under a chunk
function AllGuids($c){
  foreach($it in $c.Items){ if($it.Name -eq "InstanceGuid"){ [string]$it.InternalData } }
  foreach($sc in $c.Chunks){ AllGuids $sc }
}

# Pass 1: build component list + map every GUID (component + each param) -> component record
$comps   = @()          # ordered component records
$guid2c  = @{}          # any guid -> component record
$pName   = @{}          # param guid -> param name (for labelling)

for ($i=0; $i -lt $count; $i++) {
  $o = $objs.FindChunk("Object",$i)
  $cont = $o.FindChunk("Container")
  if (-not $cont) { continue }
  $cg = [string](Val $cont "InstanceGuid")
  $rec = [pscustomobject]@{
    Idx=$i; Name=(Val $o "Name"); Nick=(Val $cont "NickName");
    Guid=$cg; Pivot=(Val $cont "Attributes" ); Inputs=@(); ContainerSources=@(Sources $cont)
  }
  $comps += $rec
  # register every guid anywhere inside this container -> this component
  foreach ($g in (AllGuids $cont)) { if ($g) { $guid2c[$g] = $rec } }

  foreach ($pi in ($cont.Chunks | Where-Object { $_.Name -eq "param_input" })) {
    $pg = [string](Val $pi "InstanceGuid")
    if ($pg) { $guid2c[$pg] = $rec; $pName[$pg] = (Val $pi "NickName") }
    $rec.Inputs += [pscustomobject]@{ Name=(Val $pi "NickName"); Guid=$pg; Sources=@(Sources $pi) }
  }
  foreach ($po in ($cont.Chunks | Where-Object { $_.Name -eq "param_output" })) {
    $pg = [string](Val $po "InstanceGuid")
    if ($pg) { $guid2c[$pg] = $rec; $pName[$pg] = (Val $po "NickName") }
  }
}

function Label($rec){ if ($rec.Nick -and $rec.Nick -ne $rec.Name) { "$($rec.Name) [$($rec.Nick)]" } else { $rec.Name } }

# Pass 2: emit edges (source component -> this component : inputPort)
Write-Host "=== CONNECTIONS (upstream -> downstream.port) ===" -ForegroundColor Cyan
$edges = @()
foreach ($c in $comps) {
  # component-level sources (primitive params like Geometry/Point/Panel receiving input)
  foreach ($s in $c.ContainerSources) {
    $src = $guid2c[$s]
    $sl = if ($src) { Label $src } else { "??($s)" }
    $edges += [pscustomobject]@{ From=$sl; To=(Label $c); Port="(in)" }
  }
  foreach ($inp in $c.Inputs) {
    foreach ($s in $inp.Sources) {
      $src = $guid2c[$s]
      $sl = if ($src) { Label $src } else { "??($s)" }
      $edges += [pscustomobject]@{ From=$sl; To=(Label $c); Port=$inp.Name }
    }
  }
}

$edges | ForEach-Object { "{0,-40} -> {1}.{2}" -f $_.From, $_.To, $_.Port }

Write-Host "`nComponents: $($comps.Count)   Wires: $($edges.Count)" -ForegroundColor Green

# Save edge list for downstream use
$edges | Export-Csv -NoTypeInformation -Path "C:\Users\model\Documents\Github\hgh\ghx_edges.csv"
