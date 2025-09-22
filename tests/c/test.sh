#!/usr/bin/env bash

set -Eeuo pipefail

declare ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)
declare TARGET_DIR="${ROOT_DIR}/../../target/"
declare VERBOSE=""

args=$(getopt -a -o v --long verbose -- "$@")

eval set -- ${args}

while :; do
    case $1 in
    -v | --verbose)
        VERBOSE="--verbose"
        shift
        ;;
    --)
        shift
        break
        ;;
    *)
        echo >&2 "Invalid option '$1'"
        return 1
        ;;
    esac
done

pushd "${ROOT_DIR}"

if [[ ! $(find "${TARGET_DIR}" -type d ! -path "${TARGET_DIR}") ]]; then
    cargo b
fi

declare target=$(ls -dt ../../target/* | head -n 1 | xargs basename)

cmake -B build -DCMAKE_BUILD_TYPE="${target}"
cmake --build build
cd build
ctest $VERBOSE

popd
