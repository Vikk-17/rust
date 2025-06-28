/*
 * Write a Rust program that gathers system information such as the Rust version, operating system details, and system architecture.
 */

// use sysinfo::{
//     Disks, Networks, System,
// };

use sysinfo::System;

fn main() {
    let mut sys = System::new();
    
    // scope
    loop {
        // Refresh cpu usage
        sys.refresh_cpu_usage();
        for cpu in sys.cpus() {
            println!("{}", cpu.cpu_usage());
        }
    }
}
