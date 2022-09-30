#!/usr/bin/python3

import sys

# sorts by timestamp
lines = open(sys.argv[1]).readlines()
rlines = list()
for l in lines:
    rlines.append(l.strip().split(':'))
rlines.sort(key=lambda it: int(it[1]))

# tracks creation -> move -> deletion and computes tot size
create = dict()
destroy = dict()
tot = 0
for sl in rlines:
    cmd = sl[3].split(',')
    op = cmd[0]
    obj = cmd[1]
    if op == "create":
        create[obj] = create.get(obj, 0) + 1
        val = int(cmd[3])
    elif op == "destroy":
        nd = destroy.get(obj, 0)
        if obj not in create or nd + 1 > create[obj]:
            print("WRN: trying to destroy %s but not created it. Skipping." % obj)
            continue
        destroy[obj] = nd + 1
        val = int(cmd[3])
    elif op == "move":
        pass
    tot += val
    print("%s:%d" % (':'.join(sl), tot))

# obj checks and debug info
notdest = []
for k in create:
    if k not in destroy:
        notdest.append(k)
    else:
        if create[k] < destroy[k]:
            print("ERR: different # of create [%d] and destroy [%d] for same %s" % (create[k], destroy[k], k))
for k in destroy:
    if k not in create:
        print("ERR: destroyed %s but never created?!" % k)
if len(notdest) > 0:
    print("INF: total created = %d, destroyed = %d, not destroyed = %d" % (len(create), len(destroy), len(notdest)))
