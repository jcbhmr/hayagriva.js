#!/bin/bash
set -e

generate-patch()(
  rm -rf a b pkg

  build --no-patch
  mv pkg a
  cp -r a b

  touch pkg.patch
  (cd b; patch -p1 < ../pkg.patch)

  read -p "Press ENTER once done making changes to b (a is the original)" enter

  (cd b; find . -name '*.orig' -delete )
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

build:docs() (
  build
  cd pkg
  cat <<EOF > typedoc.json
{
  "entryPoints": ["hayagriva.d.ts"],
  "skipErrorChecking": true,
  "navigationLinks": {
    "GitHub": "https://github.com/typst-community/hayagriva.js"
  }
}
EOF
  npx --yes typedoc
  npx --yes http-server -c-1 docs
)

test() (
  if [[ $1 == '--no-build' ]]; then shift; else build; fi
  node --experimental-default-type=module --test "$@"
)

test:browser() (
  build
  npx --yes vite
)

cd "$(dirname "${BASH_SOURCE[0]}")"; "$@"
