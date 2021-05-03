# python3 -m pip install psutil
import sys, psutil, time

def now_ms():
    return int(time.time_ns() / 1_000_000)

def main(argv):
    if len(argv) != 2:
        sys.exit("usage: python3 monitor_process.py <duration in seconds> <pid>")

    try: 
        duration = int(argv[0])
    except ValueError:
        sys.exit("malformed duration (not an int): %s" % argv[0])
    
    try:
        pid = int(argv[1])
    except ValueError:
        sys.exit("malformed pid (not an int): %s" % argv[1])

    try:
        process = psutil.Process(pid)
    except psutil.NoSuchProcess:
        sys.exit("no running process has id %s" % pid)

    ps = process.children()
    ps.append(process)

    # print("profiling processes (%s): %s" % (len(ps), ps))

    print("ELAPSED_MS,CPU_SECS,MEM_BYTES")

    # user = user cpu time in seconds
    # system = system cpu time in seconds
    # rss = resident set size of memory in bytes
    # cpu = cpu time in seconds
    # mem = resident set size of memory in bytes
    start_stats = {"cpu": 0.0, "mem": 0}

    for p in ps:
        with p.oneshot():
            p_cpu_times = p.cpu_times()
            p_memory_info = p.memory_info()
            start_stats["cpu"] += p_cpu_times.user
            start_stats["cpu"] += p_cpu_times.system
            start_stats["mem"] += p_memory_info.rss
    
    start_ms = now_ms()
    print("%s,%s,%s" % (0, 0.0, start_stats["mem"]))

    remaining_duration = duration
    
    while remaining_duration > 0:
        time.sleep(0.9973)
        remaining_duration -= 1
        stats = {"cpu": 0.0, "mem": 0}
        elapsed_ms = now_ms() - start_ms
        for p in ps:
            with p.oneshot():
                p_cpu_times = p.cpu_times()
                p_memory_info = p.memory_info()
                stats["cpu"] += p_cpu_times.user
                stats["cpu"] += p_cpu_times.system
                stats["mem"] += p_memory_info.rss

        # "cpu" is always an aggregate of all the cpu time a process as consumed
        # since it was first started, which is why we subtract from the start value,
        # but "mem" represents the memory CURRENTLY used by the process, so we leave it as-is
        print("%s,%s,%s" % (elapsed_ms, stats["cpu"] - start_stats["cpu"], stats["mem"]))

if __name__ == "__main__":
   main(sys.argv[1:])