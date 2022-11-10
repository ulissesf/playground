#!/bin/bash

function run_child()
{
	sleep 10
	cd /home/ufurquim/Downloads/Unigine_Valley-1.0/bin
	exec ./valley_x64 -project_name Valley -data_path ../ -engine_config ../data/valley_1.0.cfg -system_script valley/unigine.cpp -sound_app openal -video_app opengl -video_multisample 1 -video_fullscreen 0 -video_mode 3 -extern_define ,RELEASE,LANGUAGE_EN,QUALITY_MEDIUM -extern_plugin ,GPUMonitor
}

run_child >& /dev/null &
chid=$!
KSRC=./linux-tree ./i915-memlog.bt $! 10 &
wait $chid
sleep 10
kill $!
