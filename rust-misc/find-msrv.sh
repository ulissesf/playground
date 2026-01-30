#!/bin/bash

TMPF=$(mktemp) || exit 1
trap "rm -f $TMPF" EXIT HUP INT TERM

>$TMPF

while read pkg_info; do
    pkg=$(echo $pkg_info | cut -d" " -f1)
    rver=$(echo $pkg_info | cut -d" " -f2 | cut -dv -f2)
    echo "=== PKG: $pkg, VERSION: $rver"
    pkg_msrv=$(cargo info --locked $pkg@$rver | grep rust-version | cut -d" " -f2)
    echo "===> MSRV: $pkg_msrv"
    if [ "$pkg_msrv" != "unknown" ]; then
        echo $pkg_msrv >> $TMPF
    fi
done < <(cargo tree --locked --prefix none | tail -n +2)

MSRV=$(sort -u $TMPF | tail -n 1)
echo ">>>"
echo ">>> MSRV: $MSRV"
echo ">>>"
