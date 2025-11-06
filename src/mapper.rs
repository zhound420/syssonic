use crate::metrics::SystemMetrics;
use tunes::prelude::*;

/// Musical parameters derived from system metrics
#[derive(Debug, Clone)]
pub struct MusicalParams {
    // Original CPU-based melody
    pub melody_notes: Vec<f32>,      // Frequencies for melody
    pub bass_note: f32,               // Bass frequency
    pub bass_velocity: f32,           // 0.0-1.0
    pub rhythm_density: f32,          // 0.0-1.0 (how many percussion hits)
    pub tempo: f32,                   // BPM
    pub filter_cutoff: f32,           // Hz
    pub reverb_mix: f32,              // 0.0-1.0
    pub kick_hits: Vec<usize>,        // Which 16th notes get kicks
    pub snare_hits: Vec<usize>,       // Which 16th notes get snares

    // GPU voice and effects
    pub gpu_notes: Option<Vec<f32>>,  // GPU-driven melody (separate voice)
    pub gpu_intensity: f32,           // 0.0-1.0 (utilization)
    pub gpu_chorus_depth: f32,        // 0.0-1.0
    pub gpu_flanger_rate: f32,        // Hz

    // GPU memory → reverb
    pub vram_reverb_size: f32,        // 0.0-1.0 (% VRAM used)

    // Load average → complexity
    pub rhythm_polyrhythm_factor: f32, // 0.0-1.0 (how polyrhythmic)
    pub harmonic_voices: usize,        // Number of additional voices

    // Swap → distortion
    pub swap_distortion: f32,         // 0.0-1.0

    // Battery → dynamics
    pub battery_volume_mult: f32,     // 0.5-1.0 (volume multiplier)
    pub battery_tonality: f32,        // -1.0 (minor) to 1.0 (major)

    // Per-core → polyrhythm patterns
    pub core_patterns: Vec<Vec<usize>>, // Rhythmic pattern per core

    // Process count → hi-hat density
    pub hihat_density: f32,           // 0.0-1.0

    // Top processes → mini-melodies
    pub process_melodies: Vec<(String, Vec<f32>)>, // (name, melody)

    // Fan speeds → ambience
    pub fan_noise_level: f32,         // 0.0-1.0
}

pub struct MetricsMapper {
    // Musical constants
    base_tempo: f32,
    scale: Vec<f32>, // Minor pentatonic by default
}

impl MetricsMapper {
    pub fn new() -> Self {
        // A minor pentatonic scale (A, C, D, E, G)
        let scale = vec![
            A3, C4, D4, E4, G4,
            A4, C5, D5, E5, G5,
            A5, C6, D6,
        ];

        Self {
            base_tempo: 90.0,
            scale,
        }
    }

    pub fn map(&self, metrics: &SystemMetrics) -> MusicalParams {
        // CPU Usage → Melody Pitch
        // Map 0-100% to our scale indices
        let scale_index = ((metrics.cpu_usage / 100.0) * (self.scale.len() - 1) as f32) as usize;
        let scale_index = scale_index.min(self.scale.len() - 1);
        
        // Create a 4-note melody pattern based on CPU
        let melody_notes = vec![
            self.scale[scale_index],
            self.scale[scale_index.saturating_sub(1).max(0)],
            self.scale[(scale_index + 2).min(self.scale.len() - 1)],
            self.scale[scale_index],
        ];

        // Memory Usage → Bass Intensity
        let bass_note = if metrics.memory_usage > 75.0 {
            A2 // Lower bass when memory is high (more ominous)
        } else if metrics.memory_usage > 50.0 {
            A2 * 1.5 // Mid bass
        } else {
            E3 // Higher bass when memory is comfortable
        };
        let bass_velocity = (metrics.memory_usage / 100.0).clamp(0.3, 1.0);

        // Disk I/O → Rhythm Density
        // Convert bytes/sec to a density metric (0.0-1.0)
        let total_disk_io = (metrics.disk_read_bytes + metrics.disk_write_bytes) as f32;
        let io_normalized = (total_disk_io / 10_000_000.0).clamp(0.0, 1.0); // 10MB/s = full density
        
        // Network Traffic → Tempo Modulation
        let total_network = (metrics.network_rx_bytes + metrics.network_tx_bytes) as f32;
        let network_normalized = (total_network / 5_000_000.0).clamp(0.0, 1.0); // 5MB/s = max tempo
        let tempo = self.base_tempo + (network_normalized * 40.0); // 90-130 BPM range

        // Temperature → Filter & Reverb
        // 30°C = closed/dry, 70°C = open/wet
        let temp_normalized = ((metrics.temperature - 30.0) / 40.0).clamp(0.0, 1.0);
        let filter_cutoff = 400.0 + (temp_normalized * 2600.0); // 400Hz - 3000Hz
        let reverb_mix = temp_normalized * 0.5; // 0% - 50% reverb

        // Generate percussion patterns based on I/O
        let (kick_hits, snare_hits) = self.generate_rhythm_pattern(
            metrics.disk_read_bytes,
            metrics.disk_write_bytes,
            io_normalized,
        );

        // === NEW MAPPINGS ===

        // GPU → Separate voice with unique scale (Dorian mode for contrast)
        let (gpu_notes, gpu_intensity, gpu_chorus_depth, gpu_flanger_rate, vram_reverb_size) =
            self.map_gpu_metrics(metrics);

        // Load average → Polyrhythmic complexity
        let (rhythm_polyrhythm_factor, harmonic_voices) = self.map_load_average(metrics);

        // Swap → Bass distortion
        let swap_distortion = self.map_swap_usage(metrics);

        // Battery → Volume and tonality
        let (battery_volume_mult, battery_tonality) = self.map_battery(metrics);

        // Per-core CPU → Polyrhythmic patterns
        let core_patterns = self.map_per_core_cpu(&metrics.per_core_usage);

        // Process count → Hi-hat density
        let hihat_density = self.map_process_count(metrics.process_count);

        // Top processes → Mini-melodies
        let process_melodies = self.map_top_processes(&metrics.top_processes);

        // Fan speeds → Ambient noise level
        let fan_noise_level = self.map_fan_speeds(metrics);

        MusicalParams {
            // Original params
            melody_notes,
            bass_note,
            bass_velocity,
            rhythm_density: io_normalized,
            tempo,
            filter_cutoff,
            reverb_mix,
            kick_hits,
            snare_hits,

            // New params
            gpu_notes,
            gpu_intensity,
            gpu_chorus_depth,
            gpu_flanger_rate,
            vram_reverb_size,
            rhythm_polyrhythm_factor,
            harmonic_voices,
            swap_distortion,
            battery_volume_mult,
            battery_tonality,
            core_patterns,
            hihat_density,
            process_melodies,
            fan_noise_level,
        }
    }

    fn generate_rhythm_pattern(
        &self,
        disk_read: u64,
        disk_write: u64,
        density: f32,
    ) -> (Vec<usize>, Vec<usize>) {
        // Base patterns (16th note grid)
        let mut kicks = vec![0, 4, 8, 12]; // Standard 4-on-floor
        let mut snares = vec![4, 12]; // Backbeat

        // Add complexity based on density
        if density > 0.3 {
            kicks.push(2);
            kicks.push(10);
        }
        if density > 0.6 {
            snares.push(6);
            snares.push(14);
        }
        if density > 0.8 {
            kicks.push(1);
            kicks.push(3);
            kicks.push(9);
            kicks.push(11);
        }

        // Reads influence kicks, writes influence snares
        if disk_read > disk_write {
            kicks.push(15);
        } else if disk_write > disk_read {
            snares.push(15);
        }

        kicks.sort();
        kicks.dedup();
        snares.sort();
        snares.dedup();

        (kicks, snares)
    }

    // === NEW MAPPING METHODS ===

    fn map_gpu_metrics(&self, metrics: &SystemMetrics) -> (Option<Vec<f32>>, f32, f32, f32, f32) {
        // Check for NVIDIA GPU first, then AMD
        let gpu_util = metrics.gpu_nvidia.as_ref().map(|g| g.utilization)
            .or_else(|| metrics.gpu_amd.as_ref().map(|g| g.utilization))
            .unwrap_or(0.0);

        let gpu_temp = metrics.gpu_nvidia.as_ref().map(|g| g.temperature)
            .or_else(|| metrics.gpu_amd.as_ref().map(|g| g.temperature))
            .unwrap_or(45.0);

        let gpu_mem_used = metrics.gpu_nvidia.as_ref().map(|g| g.memory_used)
            .or_else(|| metrics.gpu_amd.as_ref().map(|g| g.memory_used))
            .unwrap_or(0);

        let gpu_mem_total = metrics.gpu_nvidia.as_ref().map(|g| g.memory_total)
            .or_else(|| metrics.gpu_amd.as_ref().map(|g| g.memory_total))
            .unwrap_or(1);

        // If no GPU present, return None for notes
        if gpu_util < 0.1 && gpu_mem_used == 0 {
            return (None, 0.0, 0.0, 0.0, 0.0);
        }

        // GPU utilization → Dorian mode melody (for contrast with CPU's minor pentatonic)
        let dorian_scale = vec![D4, E4, F4, G4, A4, B4, C5, D5, E5, F5];
        let gpu_scale_index = ((gpu_util / 100.0) * (dorian_scale.len() - 1) as f32) as usize;
        let gpu_scale_index = gpu_scale_index.min(dorian_scale.len() - 1);

        let gpu_notes = vec![
            dorian_scale[gpu_scale_index],
            dorian_scale[(gpu_scale_index + 2).min(dorian_scale.len() - 1)],
            dorian_scale[gpu_scale_index.saturating_sub(1)],
            dorian_scale[gpu_scale_index],
        ];

        let gpu_intensity = (gpu_util / 100.0).clamp(0.0, 1.0);

        // GPU temp → Chorus and flanger effects
        let temp_norm = ((gpu_temp - 40.0) / 40.0).clamp(0.0, 1.0); // 40-80°C range
        let gpu_chorus_depth = temp_norm * 0.3; // 0-30% chorus depth
        let gpu_flanger_rate = 0.5 + (temp_norm * 2.5); // 0.5-3.0 Hz flanger

        // GPU memory → Reverb room size
        let vram_reverb_size = (gpu_mem_used as f32 / gpu_mem_total as f32).clamp(0.0, 1.0);

        (Some(gpu_notes), gpu_intensity, gpu_chorus_depth, gpu_flanger_rate, vram_reverb_size)
    }

    fn map_load_average(&self, metrics: &SystemMetrics) -> (f32, usize) {
        // Load average 1-min vs 15-min indicates load trend
        // Higher 1-min relative to 15-min = increasing load = more polyrhythmic
        let load_diff = metrics.load_avg_1 - metrics.load_avg_15;
        let load_diff_norm = (load_diff / 4.0).clamp(0.0, 1.0); // Normalize to 0-1

        let rhythm_polyrhythm_factor = load_diff_norm;

        // Number of harmonic voices based on sustained load (5-min average)
        let harmonic_voices = if metrics.load_avg_5 < 1.0 {
            1 // Minimal load
        } else if metrics.load_avg_5 < 3.0 {
            2 // Moderate load
        } else if metrics.load_avg_5 < 6.0 {
            3 // High load
        } else {
            4 // Very high load
        };

        (rhythm_polyrhythm_factor, harmonic_voices)
    }

    fn map_swap_usage(&self, metrics: &SystemMetrics) -> f32 {
        if metrics.swap_total == 0 {
            return 0.0; // No swap configured
        }

        let swap_percent = (metrics.swap_used as f32 / metrics.swap_total as f32) * 100.0;

        // Light swap (< 20%) → minimal distortion
        // Heavy swap (> 50%) → aggressive distortion
        let swap_distortion = if swap_percent < 20.0 {
            (swap_percent / 20.0) * 0.2 // 0-0.2
        } else {
            0.2 + ((swap_percent - 20.0) / 80.0) * 0.8 // 0.2-1.0
        };

        swap_distortion.clamp(0.0, 1.0)
    }

    fn map_battery(&self, metrics: &SystemMetrics) -> (f32, f32) {
        let battery = match &metrics.battery {
            Some(b) => b,
            None => return (1.0, 0.0), // No battery = default volume, neutral tonality
        };

        // State of charge → Volume multiplier (0.5x at 0%, 1.0x at 100%)
        let battery_volume_mult = 0.5 + (battery.state_of_charge / 100.0) * 0.5;

        // Charging/discharging → Tonality bias
        use crate::metrics::BatteryState;
        let battery_tonality = match battery.state {
            BatteryState::Charging => 0.3,      // Slightly major/brighter
            BatteryState::Discharging => {
                if battery.state_of_charge < 20.0 {
                    -0.7 // Very minor/dark when low
                } else {
                    -0.3 // Slightly minor
                }
            }
            BatteryState::Full => 0.5,           // Bright and happy
            BatteryState::Empty => -1.0,         // Very dark
            BatteryState::Unknown => 0.0,        // Neutral
        };

        (battery_volume_mult, battery_tonality)
    }

    fn map_per_core_cpu(&self, per_core_usage: &[f32]) -> Vec<Vec<usize>> {
        // Generate rhythmic pattern for each core
        // Higher usage = denser pattern
        per_core_usage.iter().enumerate().map(|(core_idx, &usage)| {
            let usage_norm = usage / 100.0;

            // Base pattern varies by core number (for variety)
            let offset = core_idx % 4;

            if usage_norm < 0.2 {
                // Low usage: sparse hits
                vec![offset * 4]
            } else if usage_norm < 0.5 {
                // Medium usage: quarter notes
                vec![offset * 4, (offset * 4 + 8) % 16]
            } else if usage_norm < 0.8 {
                // High usage: eighth notes
                vec![offset * 4, (offset * 4 + 4) % 16, (offset * 4 + 8) % 16, (offset * 4 + 12) % 16]
            } else {
                // Very high usage: sixteenth notes
                (0..16).filter(|i| i % 2 == offset % 2).collect()
            }
        }).collect()
    }

    fn map_process_count(&self, process_count: usize) -> f32 {
        // Normalize process count to 0-1
        // Assuming 50-300 processes is typical range
        let count_norm = ((process_count as f32 - 50.0) / 250.0).clamp(0.0, 1.0);
        count_norm
    }

    fn map_top_processes(&self, top_processes: &[crate::metrics::ProcessMetric]) -> Vec<(String, Vec<f32>)> {
        // Generate mini-melody for each top process
        let process_scale = vec![E5, G5, A5, B5, D6]; // Higher register for process melodies

        top_processes.iter().map(|proc| {
            // CPU % determines pitch
            let cpu_norm = (proc.cpu_usage / 100.0).clamp(0.0, 1.0);
            let scale_idx = (cpu_norm * (process_scale.len() - 1) as f32) as usize;

            // Create simple 2-note melodic phrase
            let melody = vec![
                process_scale[scale_idx],
                process_scale[(scale_idx + 2).min(process_scale.len() - 1)],
            ];

            (proc.name.clone(), melody)
        }).collect()
    }

    fn map_fan_speeds(&self, metrics: &SystemMetrics) -> f32 {
        let fan_speeds = match &metrics.fan_speeds {
            Some(fans) if !fans.is_empty() => fans,
            _ => return 0.0, // No fans detected
        };

        // Average RPM across all fans
        let avg_rpm = fan_speeds.iter().map(|f| f.rpm as f32).sum::<f32>() / fan_speeds.len() as f32;

        // Typical fan range: 500-3000 RPM
        // Map to 0-1
        let fan_norm = ((avg_rpm - 500.0) / 2500.0).clamp(0.0, 1.0);

        fan_norm
    }

    pub fn print_mapping_info(&self, metrics: &SystemMetrics, params: &MusicalParams) {
        println!("\n=== System Metrics → Music Mapping ===");
        println!("CPU Usage:      {:.1}% → Melody pitch (scale index)", metrics.cpu_usage);
        println!("Memory Usage:   {:.1}% → Bass intensity: {:.2}", metrics.memory_usage, params.bass_velocity);
        println!("Disk I/O:       {} KB/s → Rhythm density: {:.2}", 
            (metrics.disk_read_bytes + metrics.disk_write_bytes) / 1024, 
            params.rhythm_density);
        println!("Network:        {} KB/s → Tempo: {:.1} BPM", 
            (metrics.network_rx_bytes + metrics.network_tx_bytes) / 1024,
            params.tempo);
        println!("Temperature:    {:.1}°C → Filter: {:.0}Hz, Reverb: {:.0}%", 
            metrics.temperature,
            params.filter_cutoff,
            params.reverb_mix * 100.0);
        println!("Kick hits:      {:?}", params.kick_hits);
        println!("Snare hits:     {:?}", params.snare_hits);
        println!("=====================================\n");
    }
}
