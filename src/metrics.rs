use sysinfo::{System, Networks, Disks, Components};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub cpu_usage: f32,          // 0-100%
    pub memory_usage: f32,       // 0-100%
    pub disk_read_bytes: u64,    // bytes/sec
    pub disk_write_bytes: u64,   // bytes/sec
    pub network_rx_bytes: u64,   // bytes/sec
    pub network_tx_bytes: u64,   // bytes/sec
    pub temperature: f32,        // °C (average)
    pub timestamp: Instant,
}

pub struct MetricsCollector {
    system: System,
    networks: Networks,
    disks: Disks,
    components: Components,
    last_metrics: Option<SystemMetrics>,
    last_update: Instant,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            networks: Networks::new_with_refreshed_list(),
            disks: Disks::new_with_refreshed_list(),
            components: Components::new_with_refreshed_list(),
            last_metrics: None,
            last_update: Instant::now(),
        }
    }

    pub fn collect(&mut self) -> SystemMetrics {
        // Refresh all data
        self.system.refresh_cpu_all();
        self.system.refresh_memory();
        self.networks.refresh();
        self.disks.refresh();
        self.components.refresh();

        // Calculate time delta for rate calculations
        let now = Instant::now();
        let delta = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // CPU Usage (average across all cores)
        let cpu_usage = self.system.global_cpu_usage();

        // Memory Usage
        let memory_usage = (self.system.used_memory() as f32 / self.system.total_memory() as f32) * 100.0;

        // Disk I/O (calculate rates if we have previous data)
        let (disk_read_bytes, disk_write_bytes) = if let Some(prev) = &self.last_metrics {
            let read_delta = self.disks.iter()
                .map(|d| d.usage().read_bytes)
                .sum::<u64>()
                .saturating_sub(prev.disk_read_bytes);
            let write_delta = self.disks.iter()
                .map(|d| d.usage().write_bytes)
                .sum::<u64>()
                .saturating_sub(prev.disk_write_bytes);
            
            ((read_delta as f32 / delta) as u64, (write_delta as f32 / delta) as u64)
        } else {
            (0, 0)
        };

        // Network I/O (calculate rates)
        let (network_rx_bytes, network_tx_bytes) = if let Some(prev) = &self.last_metrics {
            let rx_delta = self.networks.iter()
                .map(|(_, data)| data.received())
                .sum::<u64>()
                .saturating_sub(prev.network_rx_bytes);
            let tx_delta = self.networks.iter()
                .map(|(_, data)| data.transmitted())
                .sum::<u64>()
                .saturating_sub(prev.network_tx_bytes);
            
            ((rx_delta as f32 / delta) as u64, (tx_delta as f32 / delta) as u64)
        } else {
            (0, 0)
        };

        // Temperature (average across all sensors)
        let temperatures: Vec<f32> = self.components.iter()
            .filter_map(|c| c.temperature().map(|t| t as f32))
            .collect();
        let temperature = if !temperatures.is_empty() {
            temperatures.iter().sum::<f32>() / temperatures.len() as f32
        } else {
            45.0 // Default fallback
        };

        let metrics = SystemMetrics {
            cpu_usage,
            memory_usage,
            disk_read_bytes,
            disk_write_bytes,
            network_rx_bytes,
            network_tx_bytes,
            temperature,
            timestamp: now,
        };

        self.last_metrics = Some(metrics.clone());
        metrics
    }

    pub fn collect_smoothed(&mut self, samples: usize, interval_ms: u64) -> SystemMetrics {
        let mut accumulated = vec![];
        
        for _ in 0..samples {
            accumulated.push(self.collect());
            std::thread::sleep(Duration::from_millis(interval_ms));
        }

        // Average the samples for smoother transitions
        let cpu_avg = accumulated.iter().map(|m| m.cpu_usage).sum::<f32>() / samples as f32;
        let mem_avg = accumulated.iter().map(|m| m.memory_usage).sum::<f32>() / samples as f32;
        let temp_avg = accumulated.iter().map(|m| m.temperature).sum::<f32>() / samples as f32;
        
        // Use max for I/O metrics (more interesting musically)
        let disk_read = accumulated.iter().map(|m| m.disk_read_bytes).max().unwrap_or(0);
        let disk_write = accumulated.iter().map(|m| m.disk_write_bytes).max().unwrap_or(0);
        let net_rx = accumulated.iter().map(|m| m.network_rx_bytes).max().unwrap_or(0);
        let net_tx = accumulated.iter().map(|m| m.network_tx_bytes).max().unwrap_or(0);

        SystemMetrics {
            cpu_usage: cpu_avg,
            memory_usage: mem_avg,
            disk_read_bytes: disk_read,
            disk_write_bytes: disk_write,
            network_rx_bytes: net_rx,
            network_tx_bytes: net_tx,
            temperature: temp_avg,
            timestamp: Instant::now(),
        }
    }
}
