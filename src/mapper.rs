use crate::metrics::SystemMetrics;
use tunes::prelude::*;

/// Musical parameters derived from system metrics
#[derive(Debug, Clone)]
pub struct MusicalParams {
    pub melody_notes: Vec<f32>,      // Frequencies for melody
    pub bass_note: f32,               // Bass frequency
    pub bass_velocity: f32,           // 0.0-1.0
    pub rhythm_density: f32,          // 0.0-1.0 (how many percussion hits)
    pub tempo: f32,                   // BPM
    pub filter_cutoff: f32,           // Hz
    pub reverb_mix: f32,              // 0.0-1.0
    pub kick_hits: Vec<usize>,        // Which 16th notes get kicks
    pub snare_hits: Vec<usize>,       // Which 16th notes get snares
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

        MusicalParams {
            melody_notes,
            bass_note,
            bass_velocity,
            rhythm_density: io_normalized,
            tempo,
            filter_cutoff,
            reverb_mix,
            kick_hits,
            snare_hits,
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
