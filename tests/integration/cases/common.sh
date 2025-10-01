get_time_iso8601() {
    date "+%Y-%m-%dT%H:%M:%S"
}

get_epoch() {
    date "+%s"
}

reload_sshd() {
    # ps | grep sshd | head -n 1 | awk '{print $1}'
    kill -HUP $(cat /var/run/sshd.pid)
}

reload_rsyslogd() {
    kill -HUP $(cat /var/run/rsyslogd.pid)
}

sleep_for() {
    seconds="$1"

    perl -e "select(undef, undef, undef, $seconds);"
}

update_sshd_config() {
    if [ ! -e "test.conf" ]; then
        return 0
    fi

    cp "test.conf" /etc/ssh/sshd_config.d/
    reload_sshd
    sleep_for 0.1
}

test_ssh() {
    user="$1"
    dest="$2"
    positive=$3
    msg="${4:-""}"
    ret=0

    ssh -o StrictHostKeyChecking=accept-new \
        "${user}@${dest}" -f "true" && ret=$? || ret=$?

    if [ $positive = true ] && [ $ret -ne 0 ]; then
        echo "SSH test failed when success expected"
        ret=1
    elif [ $positive = false ] && [ $ret -eq 0 ]; then
        echo "SSH test failed when failure expected"
        ret=1
    fi

    output=$(
        grep "pam_network_filter" /var/log/auth.log |
            tail -n 1 |
            awk '{n=split($0, array, ": "); print array[n]}'
    )

    if [ ! -z "$msg" ] && [ "$msg" == "${output#*"$msg"}" ]; then
        echo "output didn't match: $output"
        ret=1
    fi

    return $ret
}

expect_eq() {
    ret=$1
    exp=$2

    if [ $ret = $exp ]; then
        return true
    else
        return false
    fi
}

expect_ne() {
    ret=$1
    exp=$2

    if [ $ret != $exp ]; then
        return true
    else
        return false
    fi
}

assert_eq() {
    ret=$1
    exp=$2

    if [ $ret = $exp ]; then
        exit 0
    else
        exit 1
    fi
}

assert_ne() {
    ret=$1
    exp=$2

    if [ $ret = $exp ]; then
        exit 1
    else
        exit 0
    fi
}
