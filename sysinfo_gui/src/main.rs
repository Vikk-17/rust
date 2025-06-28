use sysinfo::System;

struct CpuApp {
    system: System,
    usage: f32,
    usages: Vec<u32>,
    total_mem_usage: u64,
    used_mem_usage: u64,
}


impl Default for CpuApp {
    fn default() -> Self {
        
        let mut system = System::new_all();
        // Refresh the cpu to get updated the data
        system.refresh_cpu_all();
        
        let usage = system.global_cpu_usage();
        let usages = system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
        let total_mem_usage = system.total_memory();
        let used_mem_usage = system.used_memory();



        CpuApp { 
            system,
            usage,
            usages,
            total_mem_usage,
            used_mem_usage,
        }
    }
}


fn main() {
    println!("Hello, world!");
}
