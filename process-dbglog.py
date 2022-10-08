#!/usr/bin/python3

import sys

lines = open(sys.argv[1]).readlines()
rlines = list()
for l in lines:
    rlines.append(l.strip().split(':'))
#rlines.sort(key=lambda it: int(it[1])) -> needs global clock

# tracks creation -> bind -> unbind -> deletion
objs = dict()

for l in rlines:
    cmd = l[3].split(',')
    op = cmd[0]
    obj = cmd[1]
    if op == "create":
        if obj in objs:
            print("ERR: obj %s already exists!" % obj)
        else:
            objs[obj] = dict()
            objs[obj]["state"] = "unbound"
            objs[obj]["count"] = 0
    elif op == "destroy":
        if not obj in objs:
            print("ERR: destroying obj %s again?!" % obj)
        else:
            if objs[obj]["state"] != "unbound" or objs[obj]["count"] > 0:
                print("ERR: destroying obj %s but didn´t unbind all vmas (pending %d)" % (obj, objs[obj]["count"]))
            else:
                if "hasbound" not in objs[obj]:
                    print("WRN: obj %s never bound any vmas" % obj)
                del(objs[obj])
    elif op == "vmabind":
        if not obj in objs:
            print("ERR: trying to bind a vma to missing obj %s?" % obj)
        else:
            objs[obj]["hasbound"] = True
            objs[obj]["state"] = "bound"
            objs[obj]["count"] += 1
            if objs[obj]["count"] > 1:
                print("INF: obj %s bound more than one vma [%d]" % (obj, objs[obj]["count"]))
    elif op == "vmaunbind":
        if not obj in objs:
            print("ERR: trying to unbind vma to missing obj %s?" % obj)
        elif objs[obj]["state"] != "bound":
            print("ERR: trying to unbind vma but none ever bound!" % obj)
        else:
            objs[obj]["count"] -= 1
            if objs[obj]["count"] == 0:
                objs[obj]["state"] = "unbound"
    else:
        print("ERR: op %s not supported" % op)
