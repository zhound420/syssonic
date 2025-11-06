use super::system::AmdGpuMetrics;
use libamdgpu_top::{AMDGPU, DevicePath};
use std::sync::OnceLock;

// Global AMD GPU device handle (initialized once, or None if AMD GPU unavailable)
static AMD_DEVICE: OnceLock<Option<AMDGPU>> = OnceLock::new();

/// Initialize AMD GPU device (called once)
fn init_amd_gpu() -> Option<AMDGPU> {
    // Try to find first AMD GPU device
    match DevicePath::init_amdgpu_top() {
        Ok(device_paths) => {
            if let Some(path) = device_paths.first() {
                match AMDGPU::new(path.clone()) {
                    Ok(device) => {
                        println!("✅ AMD GPU detected and initialized");
                        return Some(device);
                    }
                    Err(e) => {
                        eprintln!("ℹ️  Failed to initialize AMD GPU: {} (skipping AMD metrics)", e);
                    }
                }
            } else {
                eprintln!("ℹ️  No AMD GPU devices found (skipping AMD metrics)");
            }
        }
        Err(e) => {
            eprintln!("ℹ️  AMD GPU not available: {} (skipping AMD metrics)", e);
        }
    }
    None
}

/// Collect AMD GPU metrics
pub fn collect_amd_metrics() -> Option<AmdGpuMetrics> {
    // Initialize AMD GPU once
    let device = AMD_DEVICE.get_or_init(init_amd_gpu);
    let mut device = device.as_ref()?.clone();

    // Update device stats
    if let Err(_) = device.update() {
        return None;
    }

    // GPU utilization
    let utilization = device.get_gfx_usage()
        .map(|u| u as f32)
        .unwrap_or(0.0);

    // Temperature
    let temperature = device.get_temp()
        .map(|t| t as f32)
        .unwrap_or(45.0);

    // Memory usage
    let vram_info = device.get_vram_usage();
    let memory_used = vram_info.0.vram_usage;
    let memory_total = vram_info.0.vram_size;

    // Power draw (if available)
    let power_draw = device.get_power_average()
        .map(|p| p as f32);

    Some(AmdGpuMetrics {
        utilization,
        temperature,
        memory_used,
        memory_total,
        power_draw,
    })
}
