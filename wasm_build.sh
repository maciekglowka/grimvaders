set -e

export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
cargo build --target wasm32-unknown-unknown --release

rm ./output/wasm/ -rf
mkdir ./output/wasm/
wasm-bindgen --out-dir ./output/wasm/ --target web ./target/wasm32-unknown-unknown/release/game.wasm
cp ./wasm_assets/* ./output/wasm/ -r
cd ./output/wasm && zip -r wasm.zip *
