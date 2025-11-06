use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    // Audio settings
    pub audio_device: String,
    pub volume: f32,
    pub auto_play_on_start: bool,

    // Update settings
    pub update_interval_ms: u64,
    pub sample_count: usize,

    // Musical settings
    pub base_tempo: f32,
    pub scale_type: String, // "minor_pentatonic", "major", "blues", etc.

    // UI settings
    pub theme: String,
    pub start_minimized: bool,
    pub show_3d_viz: bool,

    // System settings
    pub auto_start: bool,
    pub enable_gpu_monitoring: bool,
    pub enable_battery_monitoring: bool,
    pub enable_fan_monitoring: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            audio_device: "default".to_string(),
            volume: 0.8,
            auto_play_on_start: false,
            update_interval_ms: 16000,
            sample_count: 3,
            base_tempo: 90.0,
            scale_type: "minor_pentatonic".to_string(),
            theme: "dark".to_string(),
            start_minimized: false,
            show_3d_viz: true,
            auto_start: false,
            enable_gpu_monitoring: true,
            enable_battery_monitoring: true,
            enable_fan_monitoring: true,
        }
    }
}

impl AppConfig {
    /// Get the config file path
    fn get_config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "syssonic", "SysSonic")
            .ok_or_else(|| anyhow::anyhow!("Failed to get project directories"))?;

        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir)?;

        Ok(config_dir.join("config.toml"))
    }

    /// Load configuration from disk
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            let default = Self::default();
            default.save()?;
            return Ok(default);
        }

        let contents = fs::read_to_string(&config_path)?;
        let config: AppConfig = toml::from_str(&contents)?;

        Ok(config)
    }

    /// Save configuration to disk
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        let contents = toml::to_string_pretty(self)?;
        fs::write(&config_path, contents)?;

        Ok(())
    }

    /// Update a specific setting and save
    pub fn update<F>(&mut self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut Self),
    {
        updater(self);
        self.save()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.volume, 0.8);
        assert_eq!(config.theme, "dark");
    }

    #[test]
    fn test_save_and_load() {
        let mut config = AppConfig::default();
        config.volume = 0.5;

        config.save().unwrap();

        let loaded = AppConfig::load().unwrap();
        assert_eq!(loaded.volume, 0.5);
    }
}
