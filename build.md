### Compile from Wat to WASM using Wabt Tooling
`.\tools\include\bin\wat2wasm.exe main.wat`



### Build from rust cli
cargo run

### Convert from wat to wasm
.\tools\include\bin\wat2wasm.exe output.wat

### Run WASM using wasm-interp
.\tools\include\bin\wasm-interp.exe output.wasm --run-all-exports
