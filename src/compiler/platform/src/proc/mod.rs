/// # Max PID
///
/// `/proc/sys/kernel/pid_max` for Linux and 99999 for FreeBSD
/// But the maximum process IDs didn't specify an operating system. So here are some others:
/// - On AIX, process IDs comprise several fields, including a "process slot" and a "generation
///   count" field. The maximum possible value is 0x03FFFFFE, because the fields occupy only the
///   bottom 26 bits of an integer, and bit #0 is always zero except for process #1.
/// - On OpenBSD the maximum is 99999.
/// - On NetBSD the maximum is 30000.
///
/// Reference:
/// - [Maximum value of the Process ID](https://unix.stackexchange.com/questions/16883/what-is-the-maximum-value-of-the-process-id)
pub const PID_MAX: u32 = if cfg!(target_os = "macos") {
  99998
} else if cfg!(target_os = "linux") && cfg!(target_pointer_width = "64") {
  4194304
} else if cfg!(target_os = "linux") && cfg!(target_pointer_width = "32") {
  32768
} else if cfg!(target_family = "windows") {
  4294967295
} else if cfg!(target_os = "openbsd") {
  99999
} else if cfg!(target_os = "netbsd") {
  30000
} else {
  32768
};
