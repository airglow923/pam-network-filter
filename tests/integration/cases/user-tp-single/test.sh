#!/usr/bin/env sh

set -eu

. "../common.sh"

update_sshd_config

test_ssh "root" "localhost" true "authentication succeeded"
