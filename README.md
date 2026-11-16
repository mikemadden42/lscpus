# lscpus
List number of cpus

Prints the value of [`std::thread::available_parallelism`](https://doc.rust-lang.org/std/thread/fn.available_parallelism.html):
the number of threads this process can usefully run in parallel. This is usually the logical CPU count,
but may be lower when CPU affinity or container/cgroup CPU quotas apply.

MSRV: 1.59.0
