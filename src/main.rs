use std::env;

#[derive(Default)]
struct SysInfo {
    arch: String,
    os: String,
    os_family: String,
}

fn main() {
    let mut sysinfo = SysInfo::default();

    sysinfo.arch = env::consts::ARCH.to_owned();
    sysinfo.os = env::consts::OS.to_owned();
    sysinfo.os_family = env::consts::FAMILY.to_owned();

    println!("arch: {}", sysinfo.arch);
    println!("os: {}", sysinfo.os);
    println!("os_family: {}", sysinfo.os_family);
}
