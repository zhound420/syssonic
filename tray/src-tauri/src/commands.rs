use crate::audio_thread::{AudioCommand, AudioEvent, AudioThread};
use crate::config::AppConfig;
use crate::mapper::{MetricsMapper, MusicalParams};
use crate::metrics::{SystemMetrics, MetricsCollector};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;

// Application state shared across commands
pub struct AppState {
    pub audio_thread: Mutex<AudioThread>,
    pub metrics_collector: Mutex<MetricsCollector>,
    pub mapper: MetricsMapper,
    pub config: Mutex<AppConfig>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            audio_thread: Mutex::new(AudioThread::new()),
            metrics_collector: Mutex::new(MetricsCollector::new()),
            mapper: MetricsMapper::new(),
            config: Mutex::new(AppConfig::load().unwrap_or_default()),
        }
    }
}

// === Audio Control Commands ===

#[tauri::command]
pub fn start_audio(state: State<AppState>) -> Result<(), String> {
    let mut collector = state.metrics_collector.lock().unwrap();
    let metrics = collector.collect_smoothed(3, 200);

    let params = state.mapper.map(&metrics);

    let audio = state.audio_thread.lock().unwrap();
    audio
        .send_command(AudioCommand::Play(params, 4))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn stop_audio(state: State<AppState>) -> Result<(), String> {
    let audio = state.audio_thread.lock().unwrap();
    audio
        .send_command(AudioCommand::Stop)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn pause_audio(state: State<AppState>) -> Result<(), String> {
    let audio = state.audio_thread.lock().unwrap();
    audio
        .send_command(AudioCommand::Pause)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn resume_audio(state: State<AppState>) -> Result<(), String> {
    let audio = state.audio_thread.lock().unwrap();
    audio
        .send_command(AudioCommand::Resume)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_volume(state: State<AppState>, volume: f32) -> Result<(), String> {
    // Update config
    let mut config = state.config.lock().unwrap();
    config.volume = volume;
    config.save().map_err(|e| e.to_string())?;

    // Send to audio thread
    let audio = state.audio_thread.lock().unwrap();
    audio
        .send_command(AudioCommand::SetVolume(volume))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_audio_state(state: State<AppState>) -> Result<serde_json::Value, String> {
    let audio = state.audio_thread.lock().unwrap();
    Ok(serde_json::json!({
        "playing": audio.is_playing(),
        "volume": audio.get_volume(),
    }))
}

// === Metrics Commands ===

#[tauri::command]
pub fn get_current_metrics(state: State<AppState>) -> Result<SystemMetrics, String> {
    let mut collector = state.metrics_collector.lock().unwrap();
    Ok(collector.collect())
}

#[tauri::command]
pub fn get_musical_params(state: State<AppState>) -> Result<MusicalParams, String> {
    let mut collector = state.metrics_collector.lock().unwrap();
    let metrics = collector.collect();
    Ok(state.mapper.map(&metrics))
}

// === Export Commands ===

#[tauri::command]
pub fn export_audio(
    state: State<AppState>,
    path: String,
    format: String,
    bars: usize,
) -> Result<(), String> {
    let mut collector = state.metrics_collector.lock().unwrap();
    let metrics = collector.collect_smoothed(5, 200);
    let params = state.mapper.map(&metrics);

    let audio = state.audio_thread.lock().unwrap();
    audio
        .send_command(AudioCommand::Export {
            path: PathBuf::from(path),
            format,
            params,
            bars,
        })
        .map_err(|e| e.to_string())
}

// === Config Commands ===

#[tauri::command]
pub fn get_config(state: State<AppState>) -> Result<AppConfig, String> {
    let config = state.config.lock().unwrap();
    Ok(config.clone())
}

#[tauri::command]
pub fn save_config(state: State<AppState>, new_config: AppConfig) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    *config = new_config;
    config.save().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_config_field(
    state: State<AppState>,
    field: String,
    value: serde_json::Value,
) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();

    match field.as_str() {
        "volume" => {
            if let Some(v) = value.as_f64() {
                config.volume = v as f32;
            }
        }
        "auto_start" => {
            if let Some(v) = value.as_bool() {
                config.auto_start = v;
            }
        }
        "theme" => {
            if let Some(v) = value.as_str() {
                config.theme = v.to_string();
            }
        }
        "update_interval_ms" => {
            if let Some(v) = value.as_u64() {
                config.update_interval_ms = v;
            }
        }
        "enable_gpu_monitoring" => {
            if let Some(v) = value.as_bool() {
                config.enable_gpu_monitoring = v;
            }
        }
        _ => return Err(format!("Unknown config field: {}", field)),
    }

    config.save().map_err(|e| e.to_string())
}

// === Event Polling ===

#[tauri::command]
pub fn poll_audio_events(state: State<AppState>) -> Result<Vec<AudioEvent>, String> {
    let audio = state.audio_thread.lock().unwrap();
    Ok(audio.poll_events())
}

// === System Info ===

#[tauri::command]
pub fn get_system_info() -> Result<serde_json::Value, String> {
    Ok(serde_json::json!({
        "platform": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
    }))
}
