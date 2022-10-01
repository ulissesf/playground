#!/usr/bin/python3

import sys

# sorts by timestamp
lines = open(sys.argv[1]).readlines()
rlines = list()
for l in lines:
    rlines.append(l.strip().split(':'))
#rlines.sort(key=lambda it: int(it[1]))

# tracks creation -> move -> deletion and computes tot size
create = dict()
destroy = dict()
skipped_d = dict()
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
            skipped_d[obj] = skipped_d.get(obj, 0) + 1
            continue
        destroy[obj] = nd + 1
        val = int(cmd[3])
    elif op == "move":
        print("Not supposed to get moves")
        continue
    else:
        continue
    tot += val
    if tot < 0:
        sys.exit("ERR: total memory below zero [%d]" % tot)
    print("%s:%d" % (':'.join(sl), tot))

# obj checks and debug info
print("------------")
notdest = 0
ndest = dict()
totc = 0
totd = 0
totsk_d = 0
for k in skipped_d:
    totsk_d += skipped_d[k]
for k in create:
    totc += create[k]
    if k not in destroy:
        notdest += create[k]
        ndest[k] = create[k]
    else:
        if create[k] < destroy[k]:
            sys.exit("ERR: different # of create [%d] and destroy [%d] for same %s" % (create[k], destroy[k], k))
        totd += destroy[k]
for k in destroy:
    if k not in create:
        print("ERR: destroyed %s but never created?!" % k)
for k in skipped_d:
    if k in create and k in ndest:
        print("INF: skipped obj %s in create but not destroyed, event ordering issue?" % k)
print("INF: total created = %d, destroyed = %d (skipped = %d), not destroyed = %d" % (totc, totd, totsk_d, notdest))
