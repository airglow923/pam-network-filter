#!/usr/bin/env bash

set -Eeuo pipefail

declare ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)
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

cargo test $VERBOSE
tests/c/test.sh $VERBOSE

popd
