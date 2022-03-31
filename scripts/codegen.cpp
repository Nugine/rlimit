#include <cassert>
#include <cstdint>
#include <iostream>
#include <limits>
#include <string>
#include <sys/resource.h>
using namespace std;

#define panic(msg) (throw std::runtime_error(msg))

template <typename T> string rust_type_name() {
    if (is_same<T, uint64_t>::value) {
        return "u64";
    }
    if (is_same<T, int64_t>::value) {
        return "i64";
    }
    if (is_same<T, uint32_t>::value) {
        return "u32";
    }
    if (is_same<T, int32_t>::value) {
        return "i32";
    }
    if (is_same<T, uint8_t>::value) {
        return "u8";
    }
    panic("unknown type");
}

template <typename T> string rust_expr(T val) {
    if (is_same<T, uint64_t>::value) {
        if (val == numeric_limits<uint64_t>::max()) {
            return "u64::MAX";
        }
        return to_string(val);
    }
    if (is_same<T, uint8_t>::value) {
        if (val == numeric_limits<uint8_t>::max()) {
            return "u8::MAX";
        }
        return to_string(val);
    }
    panic("unknown type");
}

template <typename T> string rust_const_def(string name, T val) {
    string s = "pub const ";
    s += name;
    s += ": ";
    s += rust_type_name<T>();
    s += " = ";
    s += rust_expr(val);
    s += ";";
    return s;
}

template <typename T> string rust_resource_def(string name, T val) {
    uint64_t value = static_cast<uint64_t>(val);
    if (!(value < 128)) {
        panic("resource out of bound");
    }
    return rust_const_def(name, static_cast<uint8_t>(value));
}

string rust_resource_def(string name) {
    return rust_const_def(name, static_cast<uint8_t>(255));
}

int main() {
    cout << rust_const_def("RLIM_INFINITY", RLIM_INFINITY) << endl;

#if defined(RLIMIT_AS)
    cout << rust_resource_def("RLIMIT_AS", RLIMIT_AS) << endl;
#else
    cout << rust_resource_def("RLIMIT_AS") << endl;
#endif // RLIMIT_AS

#if defined(RLIMIT_CORE)
    cout << rust_resource_def("RLIMIT_CORE", RLIMIT_CORE) << endl;
#else
    cout << rust_resource_def("RLIMIT_CORE") << endl;
#endif // RLIMIT_CORE

#if defined(RLIMIT_CPU)
    cout << rust_resource_def("RLIMIT_CPU", RLIMIT_CPU) << endl;
#else
    cout << rust_resource_def("RLIMIT_CPU") << endl;
#endif // RLIMIT_CPU

#if defined(RLIMIT_DATA)
    cout << rust_resource_def("RLIMIT_DATA", RLIMIT_DATA) << endl;
#else
    cout << rust_resource_def("RLIMIT_DATA") << endl;
#endif // RLIMIT_DATA

#if defined(RLIMIT_FSIZE)
    cout << rust_resource_def("RLIMIT_FSIZE", RLIMIT_FSIZE) << endl;
#else
    cout << rust_resource_def("RLIMIT_FSIZE") << endl;
#endif // RLIMIT_FSIZE

#if defined(RLIMIT_KQUEUES)
    cout << rust_resource_def("RLIMIT_KQUEUES", RLIMIT_KQUEUES) << endl;
#else
    cout << rust_resource_def("RLIMIT_KQUEUES") << endl;
#endif // RLIMIT_KQUEUES

#if defined(RLIMIT_LOCKS)
    cout << rust_resource_def("RLIMIT_LOCKS", RLIMIT_LOCKS) << endl;
#else
    cout << rust_resource_def("RLIMIT_LOCKS") << endl;
#endif // RLIMIT_LOCKS

#if defined(RLIMIT_MEMLOCK)
    cout << rust_resource_def("RLIMIT_MEMLOCK", RLIMIT_MEMLOCK) << endl;
#else
    cout << rust_resource_def("RLIMIT_MEMLOCK") << endl;
#endif // RLIMIT_MEMLOCK

#if defined(RLIMIT_MSGQUEUE)
    cout << rust_resource_def("RLIMIT_MSGQUEUE", RLIMIT_MSGQUEUE) << endl;
#else
    cout << rust_resource_def("RLIMIT_MSGQUEUE") << endl;
#endif // RLIMIT_MSGQUEUE

#if defined(RLIMIT_NICE)
    cout << rust_resource_def("RLIMIT_NICE", RLIMIT_NICE) << endl;
#else
    cout << rust_resource_def("RLIMIT_NICE") << endl;
#endif // RLIMIT_NICE

#if defined(RLIMIT_NOFILE)
    cout << rust_resource_def("RLIMIT_NOFILE", RLIMIT_NOFILE) << endl;
#else
    cout << rust_resource_def("RLIMIT_NOFILE") << endl;
#endif // RLIMIT_NOFILE

#if defined(RLIMIT_NOVMON)
    cout << rust_resource_def("RLIMIT_NOVMON", RLIMIT_NOVMON) << endl;
#else
    cout << rust_resource_def("RLIMIT_NOVMON") << endl;
#endif // RLIMIT_NOVMON

#if defined(RLIMIT_NPROC)
    cout << rust_resource_def("RLIMIT_NPROC", RLIMIT_NPROC) << endl;
#else
    cout << rust_resource_def("RLIMIT_NPROC") << endl;
#endif // RLIMIT_NPROC

#if defined(RLIMIT_NPTS)
    cout << rust_resource_def("RLIMIT_NPTS", RLIMIT_NPTS) << endl;
#else
    cout << rust_resource_def("RLIMIT_NPTS") << endl;
#endif // RLIMIT_NPTS

#if defined(RLIMIT_NTHR)
    cout << rust_resource_def("RLIMIT_NTHR", RLIMIT_NTHR) << endl;
#else
    cout << rust_resource_def("RLIMIT_NTHR") << endl;
#endif // RLIMIT_NTHR

#if defined(RLIMIT_POSIXLOCKS)
    cout << rust_resource_def("RLIMIT_POSIXLOCKS", RLIMIT_POSIXLOCKS) << endl;
#else
    cout << rust_resource_def("RLIMIT_POSIXLOCKS") << endl;
#endif // RLIMIT_POSIXLOCKS

#if defined(RLIMIT_RSS)
    cout << rust_resource_def("RLIMIT_RSS", RLIMIT_RSS) << endl;
#else
    cout << rust_resource_def("RLIMIT_RSS") << endl;
#endif // RLIMIT_RSS

#if defined(RLIMIT_RTPRIO)
    cout << rust_resource_def("RLIMIT_RTPRIO", RLIMIT_RTPRIO) << endl;
#else
    cout << rust_resource_def("RLIMIT_RTPRIO") << endl;
#endif // RLIMIT_RTPRIO

#if defined(RLIMIT_RTTIME)
    cout << rust_resource_def("RLIMIT_RTTIME", RLIMIT_RTTIME) << endl;
#else
    cout << rust_resource_def("RLIMIT_RTTIME") << endl;
#endif // RLIMIT_RTTIME

#if defined(RLIMIT_SBSIZE)
    cout << rust_resource_def("RLIMIT_SBSIZE", RLIMIT_SBSIZE) << endl;
#else
    cout << rust_resource_def("RLIMIT_SBSIZE") << endl;
#endif // RLIMIT_SBSIZE

#if defined(RLIMIT_SIGPENDING)
    cout << rust_resource_def("RLIMIT_SIGPENDING", RLIMIT_SIGPENDING) << endl;
#else
    cout << rust_resource_def("RLIMIT_SIGPENDING") << endl;
#endif // RLIMIT_SIGPENDING

#if defined(RLIMIT_STACK)
    cout << rust_resource_def("RLIMIT_STACK", RLIMIT_STACK) << endl;
#else
    cout << rust_resource_def("RLIMIT_STACK") << endl;
#endif // RLIMIT_STACK

#if defined(RLIMIT_SWAP)
    cout << rust_resource_def("RLIMIT_SWAP", RLIMIT_SWAP) << endl;
#else
    cout << rust_resource_def("RLIMIT_SWAP") << endl;
#endif // RLIMIT_SWAP

#if defined(RLIMIT_UMTXP)
    cout << rust_resource_def("RLIMIT_UMTXP", RLIMIT_UMTXP) << endl;
#else
    cout << rust_resource_def("RLIMIT_UMTXP") << endl;
#endif // RLIMIT_UMTXP

#if defined(RLIMIT_VMEM)
    cout << rust_resource_def("RLIMIT_VMEM", RLIMIT_VMEM) << endl;
#else
    cout << rust_resource_def("RLIMIT_VMEM") << endl;
#endif // RLIMIT_VMEM

#if defined(CODEGEN64)
    cout << "pub type rlim_t = " << rust_type_name<rlim64_t>() << ";" << endl;
#else
    cout << "pub type rlim_t = " << rust_type_name<rlim_t>() << ";" << endl;
#endif

    cout << "#[repr(C)]\n"
         << "pub struct rlimit {\n"
         << "    pub rlim_cur: rlim_t,\n"
         << "    pub rlim_max: rlim_t,\n"
         << "}\n";

#if defined(CODEGEN64)
    cout << "extern \"C\" {\n"
         << "    pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;\n"
         << "    pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;\n"
         << "}\n";
    cout << "pub use self::getrlimit64 as getrlimit;\n";
    cout << "pub use self::setrlimit64 as setrlimit;\n";
#else
    cout << "extern \"C\" {\n"
         << "    pub fn getrlimit(resource: u32, rlimit: *mut rlimit) -> i32;\n"
         << "    pub fn setrlimit(resource: u32, rlimit: *const rlimit) -> i32;\n"
         << "}\n";
#endif

    return 0;
}