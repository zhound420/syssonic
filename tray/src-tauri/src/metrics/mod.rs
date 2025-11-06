// Core system metrics
mod system;
pub use system::{
    SystemMetrics, MetricsCollector,
    NvidiaGpuMetrics, AmdGpuMetrics,
    BatteryMetrics, BatteryState,
    FanMetric, ProcessMetric,
};

// Metric collection modules
mod gpu_nvidia;
mod gpu_amd;
mod battery;
mod fans;
mod processes;
