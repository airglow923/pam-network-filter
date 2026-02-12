#!/usr/bin/env bash

set -Eeuo pipefail

declare ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)
declare FILE_OPT_IPV6="/proc/sys/net/ipv6/conf/all/disable_ipv6"
declare VERBOSE=""
declare ARGS=""

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

# skip IPv6 tests if not supported or disabled
if [ ! -f "$FILE_OPT_IPV6" ] || [ $(cat "$FILE_OPT_IPV6") == "1" ]; then
    ARGS="${ARGS} --skip ipv6"
fi

cargo test $VERBOSE -- $ARGS
tests/c/test.sh $VERBOSE

popd
