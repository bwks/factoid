use std::env;

fn main() {
    let arch = env::consts::ARCH;
    let os = env::consts::OS;
    let os_family = env::consts::FAMILY;
    println!("arch: {}", arch);
    println!("os: {}", os);
    println!("os_family: {}", os_family);
}
