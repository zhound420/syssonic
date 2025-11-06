use sysinfo::{System, ProcessStatus};
use super::system::ProcessMetric;

/// Collect top N processes by CPU usage
pub fn collect_top_processes(system: &System) -> Vec<ProcessMetric> {
    const TOP_N: usize = 5;

    let mut processes: Vec<_> = system.processes()
        .iter()
        .filter(|(_, proc)| {
            // Filter out idle/sleeping processes with zero CPU
            proc.cpu_usage() > 0.1
        })
        .map(|(pid, proc)| {
            ProcessMetric {
                name: proc.name().to_string(),
                pid: pid.as_u32(),
                cpu_usage: proc.cpu_usage(),
                memory_usage: proc.memory(),
            }
        })
        .collect();

    // Sort by CPU usage (descending)
    processes.sort_by(|a, b| {
        b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)
    });

    // Take top N
    processes.truncate(TOP_N);

    processes
}
