#!/usr/bin/env sh

set -eu

/usr/sbin/rsyslogd

# https://wiki.alpinelinux.org/wiki/HOWTO_OpenSSH_2FA_with_password_and_Google_Authenticator#Unsupported_option_UsePAM
exec /usr/sbin/sshd.pam -D "$@"
