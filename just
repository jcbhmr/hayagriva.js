#!/bin/bash
set -e

generate-patch()(
  rm -rf a b pkg

  build --no-patch
  mv pkg a
  cp -r a b

  touch pkg.patch
  (cd b && patch -p1 < ../pkg.patch)

  read -p "Press ENTER once done making changes to b (a is the original)" enter

  git diff --no-index --no-prefix a b > pkg.patch || true

  rm -rf a b
)

build()(
  if [[ $1 == '--release' ]]; then dev_flag=''; else dev_flag='--dev'; fi
  wasm-pack build $dev_flag --target web

  if [[ $1 != '--no-patch' ]]; then
    (cd pkg; patch -p1 < ../pkg.patch)
  fi

  (cd pkg; npm install --omit=dev)
)

test() (
  build
  node --experimental-default-type=module --test
)

test:browser() (
  build
  npx --yes vite
)

cd "$(dirname "${BASH_SOURCE[0]}")"; "$@"
