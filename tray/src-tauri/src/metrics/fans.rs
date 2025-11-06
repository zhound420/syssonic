use super::system::FanMetric;

// Fan monitoring is Linux-only (uses sysfs hwmon interface)
#[cfg(target_os = "linux")]
pub fn collect_fan_metrics() -> Option<Vec<FanMetric>> {
    use libmedium::sensors::{Input, Sensor};
    use std::sync::OnceLock;

    // Cache initialization status
    static FAN_INIT: OnceLock<bool> = OnceLock::new();
    static mut FAN_SENSORS: Option<Vec<Sensor>> = None;

    // Initialize once
    if FAN_INIT.get().is_none() {
        FAN_INIT.set(true).ok()?;

        match libmedium::parse_hwmons() {
            Ok(hwmons) => {
                let mut fans = Vec::new();
                for hwmon in hwmons {
                    for sensor in hwmon.sensors() {
                        // Only collect fan sensors
                        if matches!(sensor.input(), Input::FanInput(_)) {
                            fans.push(sensor.clone());
                        }
                    }
                }

                if fans.is_empty() {
                    eprintln!("ℹ️  No fan sensors found (skipping fan metrics)");
                    return None;
                }

                println!("✅ Fan monitoring initialized ({} fans found)", fans.len());
                unsafe {
                    FAN_SENSORS = Some(fans);
                }
            }
            Err(e) => {
                eprintln!("ℹ️  Fan monitoring not available: {} (skipping fan metrics)", e);
                return None;
            }
        }
    }

    // Collect fan readings
    let sensors = unsafe { FAN_SENSORS.as_ref()? };
    let mut fan_metrics = Vec::new();

    for sensor in sensors {
        if let Input::FanInput(fan_input) = sensor.input() {
            if let Ok(reading) = fan_input.read() {
                fan_metrics.push(FanMetric {
                    label: sensor.label().unwrap_or("Unknown").to_string(),
                    rpm: reading as u32,
                });
            }
        }
    }

    if fan_metrics.is_empty() {
        None
    } else {
        Some(fan_metrics)
    }
}

// Stub for non-Linux platforms
#[cfg(not(target_os = "linux"))]
pub fn collect_fan_metrics() -> Option<Vec<FanMetric>> {
    // Fan monitoring not supported on this platform
    None
}
