#!/usr/bin/python3

import ctypes
import os
import struct
import time

import perf_event

# Define the syscall function for perf_event_open
syscall = ctypes.CDLL(None).syscall
PERF_EVENT_OPEN = 298  # syscall number for perf_event_open on Linux x86_64

def read_perf_config(fname):
    """Returns config needed for power perf event."""
    config = None
    umask = 0

    cfg = open(fname).readline().strip().split(",")
    for it in cfg:
        kv = it.strip().split("=")
        if kv[0].startswith("event"):
            config = int(kv[1], 16)
        elif kv[0].startswith("umask"):
            umask = int(kv[1])
        else:
            print("ERR: unknown key %s in %s perf config file" % (kv[0], fname))
            return None

    if config is None:
        print("ERR: no valid config in perf config file %s" % fname)
        return None

    return (umask << 8) | config

# Every read of perf event will need to consume 1 + nr of fds opened
#   E.g. if there are 2 fds in same group, every read needs to be of 3 u64
#   values: # evts, 1st val, 2nd val
def open_perf_event(type_, config, pid = -1, cpu = 0, group_fd = -1):
    """Opens a perf event file descriptor for the given PMU config."""
    attr = perf_event.perf_event_attr()
    attr.type = type_
    attr.size = ctypes.sizeof(attr)
    attr.config = config
    attr.sample_type = perf_event.PERF_SAMPLE_IDENTIFIER
    attr.read_format = perf_event.PERF_FORMAT_GROUP

    fd = syscall(PERF_EVENT_OPEN, ctypes.byref(attr), pid, cpu, group_fd, 0)
    if fd == -1:
        raise OSError("Failed to open perf event")
    return fd

def read_perf_event(fd, nr):
    """Returns list with 'nr' integer values."""
    buf = os.read(fd, nr * ctypes.sizeof(ctypes.c_ulonglong))
    return [val[0] for val in struct.iter_unpack("Q", buf)]

pwr_type = int(open("/sys/devices/power/type").readline().strip())
gpu_energy_cfg = read_perf_config("/sys/devices/power/events/energy-gpu")
gpu_energy_scale = float(open("/sys/devices/power/events/energy-gpu.scale").readline().strip())

fd = open_perf_event(pwr_type, gpu_energy_cfg)
last_energy = read_perf_event(fd, 2)[1]  # discarding #evts
last_update = time.monotonic()

while True:
    time.sleep(1.5)

    new_energy = read_perf_event(fd, 2)[1]  # discarding #evts
    new_update = time.monotonic()

    delta_energy = new_energy - last_energy
    delta_time = new_update - last_update

    last_energy = new_energy
    last_update = new_update

    pwr_usage = (delta_energy * gpu_energy_scale) / delta_time

    print("GPU power usage: %.1f" % pwr_usage)
