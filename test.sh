#!/usr/bin/env bash

set -Eeuo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)

pushd "${ROOT_DIR}"

cargo test
tests/c/test.sh

popd
