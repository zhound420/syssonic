use super::system::NvidiaGpuMetrics;
use nvml_wrapper::Nvml;
use std::sync::OnceLock;

// Global NVML instance (initialized once, or None if NVIDIA GPU unavailable)
static NVML_INSTANCE: OnceLock<Option<Nvml>> = OnceLock::new();

/// Initialize NVML library (called once)
fn init_nvml() -> Option<Nvml> {
    match Nvml::init() {
        Ok(nvml) => {
            println!("✅ NVIDIA GPU detected and initialized");
            Some(nvml)
        }
        Err(e) => {
            // Gracefully handle absence of NVIDIA GPU/drivers
            eprintln!("ℹ️  NVIDIA GPU not available: {} (skipping NVIDIA metrics)", e);
            None
        }
    }
}

/// Collect NVIDIA GPU metrics
pub fn collect_nvidia_metrics() -> Option<NvidiaGpuMetrics> {
    // Initialize NVML once
    let nvml = NVML_INSTANCE.get_or_init(init_nvml);

    let nvml = nvml.as_ref()?;

    // Get first device (device 0)
    // TODO: Support multiple GPUs in the future
    let device = match nvml.device_by_index(0) {
        Ok(dev) => dev,
        Err(_) => return None,
    };

    // Collect metrics (handle errors gracefully)
    let utilization = device.utilization_rates()
        .ok()
        .map(|u| u.gpu as f32)
        .unwrap_or(0.0);

    let temperature = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
        .ok()
        .map(|t| t as f32)
        .unwrap_or(45.0);

    let memory_info = device.memory_info().ok()?;
    let memory_used = memory_info.used;
    let memory_total = memory_info.total;

    let power_draw = device.power_usage()
        .ok()
        .map(|p| p as f32 / 1000.0) // Convert milliwatts to watts
        .unwrap_or(0.0);

    let fan_speed = device.fan_speed(0) // First fan
        .ok()
        .map(|f| f as f32);

    Some(NvidiaGpuMetrics {
        utilization,
        temperature,
        memory_used,
        memory_total,
        power_draw,
        fan_speed,
    })
}
