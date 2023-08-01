#!/bin/bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
EXE_CONT=/tmp/exe_cont
EXE_OUTPUT=$(mktemp)

if [[ -z "${KSRC}" ]]; then
    echo Error: Requires KSRC env var to be set to kernel source
    exit 1
fi

function run_exe()
{
    cmdline="$@"
    read notused < $EXE_CONT
    exec $cmdline
}

function run_memlog()
{
    local chk
    let chk=0
    while read ln; do
	echo $ln
	let chk++
	if [ "$chk" = "3" ]; then
	    echo "s" > $EXE_CONT
	fi
    done < <(sudo -E $SCRIPT_DIR/i915-memlog.bt $1 100)
}

if [ ! -p $EXE_CONT ]; then
    mkfifo $EXE_CONT
fi

echo MEMLOG: "$@"
run_exe "$@" >& $EXE_OUTPUT &
exe_pid=$!

echo MEMLOG: Attaching probes...
run_memlog $exe_pid &
memlog_pid=$!

wait $exe_pid
if [ $? != 0 ]; then
	echo "MEMLOG: ERROR running $@:"
	cat $EXE_OUTPUT
fi
sleep 1
sudo kill $memlog_pid
rm $EXE_CONT $EXE_OUTPUT
