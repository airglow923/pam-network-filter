#!/usr/bin/env bash

set -Eeuo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)
TARGET_DIR="${ROOT_DIR}/../../target/"

pushd "${ROOT_DIR}"

if [[ ! $(find "${TARGET_DIR}" -type d ! -path "${TARGET_DIR}") ]]; then
    cargo b
fi

declare target=$(ls -dt ../../target/* | head -n 1 | xargs basename)

cmake -B build -DCMAKE_BUILD_TYPE="${target}"
cmake --build build
cd build
ctest

popd
