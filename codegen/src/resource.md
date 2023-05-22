# Resource

# RLIMIT_AS

The maximum size (in bytes) of the process's virtual memory (address space).

# RLIMIT_CORE

The maximum size (in bytes) of a core file that the process may dump.

# RLIMIT_CPU

A limit (in seconds) on the amount of CPU time that the process can consume.

# RLIMIT_DATA

The maximum size (in bytes) of the process's data segment (initialized data, uninitialized data, and heap).

# RLIMIT_FSIZE

The maximum size (in bytes) of files that the process may create.

# RLIMIT_KQUEUES

The maximum number of kqueues this user id is allowed to create.

# RLIMIT_LOCKS

(early Linux 2.4 only)

A limit on the combined number of `flock(2)` locks and `fcntl(2)` leases that this process may establish.

# RLIMIT_MEMLOCK

The maximum number (in bytes) of memory that may be locked into RAM.

# RLIMIT_MSGQUEUE

A limit on the number of bytes that can be allocated for POSIX message queues for the real user ID of the calling process.

# RLIMIT_NICE

This specifies a ceiling to which the process's nice value can be raised using `setpriority(2)` or `nice(2)`.

# RLIMIT_NOFILE

This specifies a value one greater than the maximum file descriptor number that can be opened by this process.

# RLIMIT_NOVMON

The number of open vnode monitors.

# RLIMIT_NPROC

A limit on the number of extant process (or, more precisely on Linux, threads) for the real user ID of the calling process.

# RLIMIT_NPTS

The maximum number of pseudo-terminals this user id is allowed to create.

# RLIMIT_NTHR

The maximum number of simultaneous threads (Lightweight Processes) for this user id. 
Kernel threads and the first thread of each process are not counted against this limit.

# RLIMIT_POSIXLOCKS

The maximum number of POSIX-type advisory-mode locks available to this user.

# RLIMIT_RSS

A limit (in bytes) on the process's resident set (the number of virtual pages resident in RAM).

# RLIMIT_RTPRIO

This specifies a ceiling on the real-time priority that may be set for this process using `sched_setscheduler(2)` and `sched_setparam(2)`.

# RLIMIT_RTTIME

A limit (in microseconds) on the amount of CPU time that a process scheduled under a real-time scheduling policy may consume without making a blocking system call.

# RLIMIT_SBSIZE

The maximum size (in bytes) of socket buffer usage for this user. This limits the amount of network memory, and hence the amount of mbufs, that this user may hold at any time.

# RLIMIT_SIGPENDING

A limit on the number of signals that may be queued for the real user ID ofthe calling process.

# RLIMIT_STACK

The maximum size (in bytes) of the process stack.

# RLIMIT_SWAP

The maximum size (in bytes) of the swap space that may be reserved or used by all of this user id's processes.

# RLIMIT_THREADS

**AIX**: The maximum number of threads each process can create. This limit is enforced by the kernel and the pthread debug library.

# RLIMIT_UMTXP

The number of shared locks a given user may create simultaneously.

# RLIMIT_VMEM

An alias for [`RLIMIT_AS`](Resource::AS). The maximum size of a process's mapped address space in bytes.
