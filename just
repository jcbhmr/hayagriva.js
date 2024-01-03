#!/bin/bash
set -e

generate-patch() (
  cd hayagriva
  git diff > ../hayagriva.patch
)

patch() (
  cd hayagriva
  patch -p1 ../hayagriva.patch
)

build() (
  cd hayagriva
  cargo wasix build --features cli "$@"
)

test() (
  build
  (cd tests/wasmer-browser; npm test)
  # (cd tests/wasmer-node; npm test)
)

wasmer-publish() (
  cd wasmer
  cp ../hayagriva/target/wasm32-wasmer-wasi/release/hayagriva.wasm .
  wasmer publish "$@"
)

cd "$(dirname "${BASH_SOURCE[0]}")"; "$@"
