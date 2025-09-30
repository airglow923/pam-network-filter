#!/usr/bin/env sh

set -eu

. "../common.sh"

test_ssh "root" "localhost" true "authentication succeeded"
