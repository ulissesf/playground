#!/usr/bin/env python3

import sys, os, stat, os.path, time

class MemRegionStats(object):
    def __init__(self, name):
        self.name = name
        self.total = 0
        self.shared = 0
        self.active = 0
        self.resident = 0
        self.purgeable = 0

    def __repr__(self):
        return "MemRegionStats(name=%s, total=%d, shared=%d, active=%d, resident=%d, purgeable=%d" % \
                (self.name, self.total, self.shared, self.active, self.resident, self.purgeable)

class DrmFdinfo(object):
    def __init__(self, pid, fdinfo_path):
        self.pid = pid
        self.fdinfo_path = fdinfo_path
        self.mem_regions = dict()
        self.driver = None
        self.pdev = None
        self.id = None

    def add_mem_region(self, region, stat, rawval):
        if not region in self.mem_regions:
            self.mem_regions[region] = MemRegionStats(region)

        vlst = rawval.split()
        vnr = int(vlst[0])
        if len(vlst) > 1:
            if vlst[1] == "KiB":
                vnr *= 1024
            elif vlst[1] == "MiB":
                vnr *= 1024 * 1024
            elif vlst[1] == "GiB":
                vnr *= 1024 * 1024 * 1024

        setattr(self.mem_regions[region], stat, vnr)

    def __repr__(self):
        return "DrmFdinfo(pid=%s, fdinfo=%s, driver=%s, pdev=%s, id=%s, mem_regions=%s" % \
                (self.pid, self.fdinfo_path, self.driver, self.pdev, self.id, self.mem_regions)

def is_drm_fd(fd_path):
    if not os.path.exists(fd_path):
        return None

    st_res = os.stat(fd_path)
    if stat.S_ISCHR(st_res.st_mode) and os.major(st_res.st_rdev) == 226:
        return os.minor(st_res.st_rdev)

    return None

def parse_drm_fdinfo(pid, fdinfo_path):
    try:
        lines = open(fdinfo_path).readlines()
    except:
        return None

    ninfo = DrmFdinfo(pid, fdinfo_path)
    for l in lines:
        nl = l.strip().split(':')
        k, v = nl[0], nl[1]

        if k.startswith("drm-driver"):
            ninfo.driver = v.strip()
        elif k.startswith("drm-client-id"):
            ninfo.id = v.strip()
        elif k.startswith("drm-pdev"):
            v = ":".join(nl[1:])
            ninfo.pdev = v.strip()
        elif k.startswith("drm-total-") or k.startswith("drm-shared-") or \
             k.startswith("drm-active-") or k.startswith("drm-resident-") or \
             k.startswith("drm-purgeable-"):
                parts = k.split('-')
                region = "-".join(parts[2:])
                ninfo.add_mem_region(region, parts[1], v.strip())
    return ninfo

if len(sys.argv) == 1:
    print("ERR: missing required argument. Usage: %s <pid> [interval in ms (default=50)]" % sys.argv[0])
    sys.exit(1)
elif len(sys.argv) == 2:
    interval = 0.05 # 50 ms
elif len(sys.argv) > 2:
    interval = float(sys.argv[2]) * 0.001

base_pid = sys.argv[1]
acum_ms = 0
last_time = time.monotonic_ns()

print("i915-memlog,time (ms),res smem (bytes),res lmem (bytes)")
while os.path.isdir("/proc/%s" % base_pid):
    idx = 0
    pid_list = [base_pid,]
    mem_stats = []

    while idx < len(pid_list):
        pid = pid_list[idx]
        idx += 1

        pid_dir = "/proc/%s" % pid
        if not os.path.isdir(pid_dir):
            continue

        # add all children pids to the list
        with os.scandir("%s/task" % pid_dir) as it:
            for et in it:
                if et.is_dir():
                    try:
                        children = open("%s/task/%s/children" % (pid_dir, et.name)).readline().strip().split()
                    except:
                        continue
                    pid_list.extend(children)

        # get all DMR fds/fdinfo
        fdinfo_dir = "%s/fdinfo" % pid_dir
        if not os.path.isdir(fdinfo_dir):
            continue

        fdinfo_data = dict()
        with os.scandir(fdinfo_dir) as it:
            for et in it:
                fd = "%s/fd/%s" % (pid_dir, et.name)
                try:
                    minor = is_drm_fd(fd)
                except:
                    minor = None
                if minor is None:
                    continue

                fdinfo = "%s/%s" % (fdinfo_dir, et.name)
                info = parse_drm_fdinfo(pid, fdinfo)
                if info is not None and (minor, info.id) not in fdinfo_data:
                    fdinfo_data[(minor, info.id)] = info

        for k in fdinfo_data:
            mem_stats.append(fdinfo_data[k])
 
    # compute total smem/lmem and report results
    smem = 0
    lmem = 0
    for info in mem_stats:
        for mr in info.mem_regions.values():
            if "local" in mr.name:
                lmem += mr.resident
            elif "system" in mr.name:
                smem += mr.resident

    cur_time = time.monotonic_ns()
    acum_ms += (cur_time - last_time) / 1000000
    last_time = cur_time

    print("i915-memlog,%d,%d,%d" % (acum_ms, smem, lmem))
    time.sleep(interval)
