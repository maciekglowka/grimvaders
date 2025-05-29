set -e
export RUST_LOG="debug,wgpu=error,naga=error"
./generate_sprites.sh
cargo run
