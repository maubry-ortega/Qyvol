// # VolleyDevByMaubry [39/∞] "El sistema revela su estado real, sin máscaras."
use sysinfo::{Disks, Networks, System};

pub struct SystemInfo {
    pub total_memory: u64,
    pub used_memory: u64,
    pub cpu_brand: String,
    pub cpu_cores: usize,
    pub cpu_usage: f32,
    pub system_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String,
    pub disks: Vec<String>,
    pub networks: Vec<String>,
}

impl SystemInfo {
    pub fn gather() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let cpu_brand = sys.cpus().first().map(|c| c.brand().to_string()).unwrap_or_default();
        let cpu_cores = sys.cpus().len();
        let cpu_usage = if cpu_cores > 0 {
            sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / cpu_cores as f32
        } else {
            0.0
        };
        let system_name = System::name().unwrap_or_default();
        let kernel_version = System::kernel_version().unwrap_or_default();
        let os_version = System::os_version().unwrap_or_default();
        let host_name = System::host_name().unwrap_or_default();

        let disks = Disks::new_with_refreshed_list()
            .iter()
            .map(|d| d.name().to_string_lossy().to_string())
            .collect();
        let networks = Networks::new_with_refreshed_list().keys().cloned().collect();

        SystemInfo {
            total_memory,
            used_memory,
            cpu_brand,
            cpu_cores,
            cpu_usage,
            system_name,
            kernel_version,
            os_version,
            host_name,
            disks,
            networks,
        }
    }
}
