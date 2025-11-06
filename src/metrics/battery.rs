use super::system::{BatteryMetrics, BatteryState};
use battery::{Manager, State};
use std::sync::OnceLock;

// Global battery manager (initialized once)
static BATTERY_MANAGER: OnceLock<Option<Manager>> = OnceLock::new();

/// Initialize battery manager (called once)
fn init_battery_manager() -> Option<Manager> {
    match Manager::new() {
        Ok(manager) => {
            println!("✅ Battery monitoring initialized");
            Some(manager)
        }
        Err(e) => {
            eprintln!("ℹ️  Battery not available: {} (skipping battery metrics)", e);
            None
        }
    }
}

/// Collect battery metrics
pub fn collect_battery_metrics() -> Option<BatteryMetrics> {
    // Initialize battery manager once
    let manager = BATTERY_MANAGER.get_or_init(init_battery_manager);
    let manager = manager.as_ref()?;

    // Get first battery
    let batteries = manager.batteries().ok()?;
    let battery = batteries.into_iter().next()?.ok()?;

    // State of charge (percentage)
    let state_of_charge = battery.state_of_charge().value * 100.0;

    // Battery state
    let state = match battery.state() {
        State::Charging => BatteryState::Charging,
        State::Discharging => BatteryState::Discharging,
        State::Full => BatteryState::Full,
        State::Empty => BatteryState::Empty,
        _ => BatteryState::Unknown,
    };

    // Power rate (watts)
    // Positive when charging, negative when discharging
    let power_rate = battery.energy_rate().value;
    let power_rate = if state == BatteryState::Charging {
        power_rate.abs()
    } else {
        -power_rate.abs()
    };

    // Temperature (if available)
    let temperature = battery.temperature()
        .ok()
        .map(|t| {
            // Convert from Kelvin to Celsius
            t.value - 273.15
        });

    // Time to full (if charging)
    let time_to_full = if state == BatteryState::Charging {
        battery.time_to_full()
            .ok()
            .flatten()
            .map(|t| t.value / 60.0) // Convert seconds to minutes
    } else {
        None
    };

    // Time to empty (if discharging)
    let time_to_empty = if state == BatteryState::Discharging {
        battery.time_to_empty()
            .ok()
            .flatten()
            .map(|t| t.value / 60.0) // Convert seconds to minutes
    } else {
        None
    };

    Some(BatteryMetrics {
        state_of_charge,
        state,
        power_rate,
        temperature,
        time_to_full,
        time_to_empty,
    })
}
