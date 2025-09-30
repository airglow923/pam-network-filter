#!/usr/bin/env sh

set -eu

DIR_ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)
DIR_TEST_CASES="cases"

[ -e "/etc/pam.d/sshd.bak" ] || cp /etc/pam.d/sshd /etc/pam.d/sshd.bak

cd "${DIR_ROOT}"

for test_case in $(find "$DIR_TEST_CASES" -maxdepth 1 -type d ! -path "$DIR_TEST_CASES"); do
    echo "Test case: $(basename "$test_case") running"
    cd "$test_case"

    cp /etc/pam.d/sshd /etc/pam.d/sshd.old
    cp sshd /etc/pam.d/sshd
    sh "./test.sh" && ret=$? || ret=$?

    printf "Test case: $(basename "$test_case") "

    if [ $ret -ne 0 ]; then
        echo "failed"
    else
        echo "succeeded"
    fi

    cd - >/dev/null 2>&1
done
