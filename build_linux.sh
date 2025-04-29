docker run --rm \
  -v $(pwd):/app \
  -v $(pwd)/../rogalik:/rogalik \
  -v $(pwd)/../wunderkammer:/wunderkammer \
  -v cargo_index:/usr/local/cargo \
  -e ROGALIK_ASSETS=/app/assets \
  --user $(id -u):$(id -g) \
  -t rust_linux
