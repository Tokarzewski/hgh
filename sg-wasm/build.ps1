# Build sg-wasm to WebAssembly + JS bindings.
# Prereqs (one-time):
#   rustup target add wasm32-unknown-unknown
#   cargo install wasm-bindgen-cli
$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

cargo build --release --target wasm32-unknown-unknown
$wasm = "target/wasm32-unknown-unknown/release/sg_wasm.wasm"

wasm-bindgen --target web    --out-dir pkg      $wasm   # ES module for the web app
wasm-bindgen --target nodejs --out-dir pkg-node $wasm   # CommonJS for the Node test

Write-Host "built pkg/ (web) and pkg-node/ (node); wasm = $((Get-Item pkg/sg_wasm_bg.wasm).Length) bytes"
