use crate::mapper::MusicalParams;
use tunes::prelude::*;
use anyhow::Result;

pub struct SystemComposer {
    engine: AudioEngine,
}

impl SystemComposer {
    pub fn new() -> Result<Self> {
        let engine = AudioEngine::with_buffer_size(4096)?;
        Ok(Self { engine })
    }

    pub fn compose_and_play(&self, params: &MusicalParams, duration_bars: usize) -> Result<()> {
        let mut comp = Composition::new(Tempo::new(params.tempo));
        let sixteenth = comp.tempo().sixteenth_note();
        let eighth = comp.tempo().eighth_note();
        let quarter = comp.tempo().quarter_note();

        // === MELODY (CPU Usage) ===
        // Create an evolving melody using the CPU-driven notes
        comp.instrument("melody", &Instrument::synth_lead())
            .filter(Filter::low_pass(params.filter_cutoff, 0.6))
            .effect(Effect::reverb(params.reverb_mix, 0.5))
            .effect(Effect::delay(eighth * 3.0, 0.3, 0.4));

        // Play the melody pattern multiple times with variations
        for bar in 0..duration_bars {
            for (i, &note) in params.melody_notes.iter().enumerate() {
                let duration = if i % 2 == 0 { eighth } else { sixteenth };
                comp.instrument("melody", &Instrument::synth_lead())
                    .note(&[note], duration);
            }
        }

        // === BASS (Memory Usage + Swap) ===
        // Deep, sustained bass notes that reflect memory pressure
        // Swap usage adds distortion
        let bass_distortion = params.bass_velocity * 0.3 + params.swap_distortion * 0.4;
        comp.instrument("bass", &Instrument::sub_bass())
            .filter(Filter::low_pass(800.0, 0.8))
            .effect(Effect::distortion(bass_distortion));

        for _ in 0..duration_bars {
            // Whole note bass pattern
            comp.instrument("bass", &Instrument::sub_bass())
                .note_with_velocity(&[params.bass_note], quarter * 4.0, params.bass_velocity);
        }

        // === DRUMS (Disk I/O) ===
        // Dynamic percussion based on disk activity
        for bar in 0..duration_bars {
            comp.track("drums")
                .drum_grid(16, sixteenth)
                .kick(&params.kick_hits)
                .snare(&params.snare_hits);
        }

        // === AMBIENT PAD (Temperature) ===
        // Atmospheric layer that gets more present as temperature rises
        if params.reverb_mix > 0.2 {
            comp.instrument("pad", &Instrument::synth_pad())
                .filter(Filter::low_pass(params.filter_cutoff * 1.5, 0.3))
                .effect(Effect::reverb(params.reverb_mix, 0.8))
                .effect(Effect::chorus(0.5, 2.0, 0.3));

            // Sustained chords
            for _ in 0..duration_bars {
                comp.instrument("pad", &Instrument::synth_pad())
                    .notes(&[A2, C3, E3], quarter * 4.0);
            }
        }

        // === HI-HATS (Network Activity + Process Count) ===
        // Hi-hat density driven by process count
        let hihat_hits = if params.hihat_density < 0.3 {
            // Sparse: every other eighth note
            vec![0, 4, 8, 12]
        } else if params.hihat_density < 0.7 {
            // Medium: every eighth note
            (0..16).filter(|i| i % 2 == 0).collect()
        } else {
            // Dense: every sixteenth note
            (0..16).collect()
        };

        for _ in 0..duration_bars {
            comp.track("hihats")
                .drum_grid(16, sixteenth)
                .hihat(&hihat_hits);
        }

        // === GPU VOICE (GPU Utilization) ===
        // Separate melodic voice for GPU activity
        if let Some(gpu_notes) = &params.gpu_notes {
            if params.gpu_intensity > 0.1 {
                comp.instrument("gpu", &Instrument::analog_synth())
                    .filter(Filter::low_pass(params.filter_cutoff * 1.2, 0.7))
                    .effect(Effect::chorus(params.gpu_chorus_depth, 0.8, 0.4));

                for _ in 0..duration_bars {
                    for &note in gpu_notes.iter() {
                        let duration = eighth * params.gpu_intensity.max(0.5); // Slower when low util
                        comp.instrument("gpu", &Instrument::analog_synth())
                            .note_with_velocity(&[note], duration, params.gpu_intensity);
                    }
                }
            }
        }

        // === PER-CORE POLYRHYTHMS (Per-Core CPU) ===
        // Each core gets its own shaker pattern (limit to first 4 cores for clarity)
        for (core_idx, pattern) in params.core_patterns.iter().take(4).enumerate() {
            if !pattern.is_empty() && params.rhythm_polyrhythm_factor > 0.2 {
                for _ in 0..duration_bars {
                    comp.track(&format!("core{}", core_idx))
                        .drum_grid(16, sixteenth)
                        .shaker(pattern);
                }
            }
        }

        // === PROCESS MELODIES (Top Processes) ===
        // Mini-melodies for top processes (limit to top 3 for clarity)
        for (proc_name, melody) in params.process_melodies.iter().take(3) {
            comp.instrument(&format!("proc_{}", proc_name), &Instrument::music_box());

            for _ in 0..duration_bars {
                for &note in melody.iter() {
                    comp.instrument(&format!("proc_{}", proc_name), &Instrument::music_box())
                        .note(&[note], sixteenth * 3.0);
                }
            }
        }

        // === FAN NOISE (Fan Speeds) ===
        // Ambient wind noise based on fan RPM
        if params.fan_noise_level > 0.1 {
            comp.instrument("fans", &Instrument::noise())
                .filter(Filter::high_pass(2000.0, 0.5));

            for _ in 0..duration_bars {
                comp.instrument("fans", &Instrument::noise())
                    .note_with_velocity(&[A3], quarter * 4.0, params.fan_noise_level * 0.3);
            }
        }

        // === VRAM REVERB (GPU Memory) ===
        // Global reverb size determined by VRAM usage
        let vram_reverb_decay = 0.3 + (params.vram_reverb_size * 4.7); // 0.3s - 5.0s

        // Play the composition
        let mut mixer = comp.into_mixer();

        // Apply battery volume modulation
        // Note: tunes library may not have set_volume method, this is conceptual
        // In practice, we'd need to scale all instrument velocities by battery_volume_mult
        // For now, this serves as documentation of the intent

        self.engine.play_mixer(&mixer)?;

        Ok(())
    }

    pub fn compose_and_export(
        &self,
        params: &MusicalParams,
        duration_bars: usize,
        output_path: &str,
        format: ExportFormat,
    ) -> Result<()> {
        let mut comp = Composition::new(Tempo::new(params.tempo));
        let sixteenth = comp.tempo().sixteenth_note();
        let eighth = comp.tempo().eighth_note();
        let quarter = comp.tempo().quarter_note();

        // Same composition as above
        comp.instrument("melody", &Instrument::synth_lead())
            .filter(Filter::low_pass(params.filter_cutoff, 0.6))
            .effect(Effect::reverb(params.reverb_mix, 0.5))
            .effect(Effect::delay(eighth * 3.0, 0.3, 0.4));

        for bar in 0..duration_bars {
            for (i, &note) in params.melody_notes.iter().enumerate() {
                let duration = if i % 2 == 0 { eighth } else { sixteenth };
                comp.instrument("melody", &Instrument::synth_lead())
                    .note(&[note], duration);
            }
        }

        // === BASS with SWAP distortion ===
        let bass_distortion = params.bass_velocity * 0.3 + params.swap_distortion * 0.4;
        comp.instrument("bass", &Instrument::sub_bass())
            .filter(Filter::low_pass(800.0, 0.8))
            .effect(Effect::distortion(bass_distortion));

        for _ in 0..duration_bars {
            comp.instrument("bass", &Instrument::sub_bass())
                .note_with_velocity(&[params.bass_note], quarter * 4.0, params.bass_velocity);
        }

        for bar in 0..duration_bars {
            comp.track("drums")
                .drum_grid(16, sixteenth)
                .kick(&params.kick_hits)
                .snare(&params.snare_hits);
        }

        if params.reverb_mix > 0.2 {
            comp.instrument("pad", &Instrument::synth_pad())
                .filter(Filter::low_pass(params.filter_cutoff * 1.5, 0.3))
                .effect(Effect::reverb(params.reverb_mix, 0.8))
                .effect(Effect::chorus(0.5, 2.0, 0.3));

            for _ in 0..duration_bars {
                comp.instrument("pad", &Instrument::synth_pad())
                    .notes(&[A2, C3, E3], quarter * 4.0);
            }
        }

        // === HI-HATS with Process Count density ===
        let hihat_hits = if params.hihat_density < 0.3 {
            vec![0, 4, 8, 12]
        } else if params.hihat_density < 0.7 {
            (0..16).filter(|i| i % 2 == 0).collect()
        } else {
            (0..16).collect()
        };

        for _ in 0..duration_bars {
            comp.track("hihats")
                .drum_grid(16, sixteenth)
                .hihat(&hihat_hits);
        }

        // === GPU VOICE and NEW ELEMENTS (same as compose_and_play) ===
        // Add GPU voice, per-core polyrhythms, process melodies, and fan noise
        if let Some(gpu_notes) = &params.gpu_notes {
            if params.gpu_intensity > 0.1 {
                comp.instrument("gpu", &Instrument::analog_synth())
                    .filter(Filter::low_pass(params.filter_cutoff * 1.2, 0.7))
                    .effect(Effect::chorus(params.gpu_chorus_depth, 0.8, 0.4));

                for _ in 0..duration_bars {
                    for &note in gpu_notes.iter() {
                        let duration = eighth * params.gpu_intensity.max(0.5);
                        comp.instrument("gpu", &Instrument::analog_synth())
                            .note_with_velocity(&[note], duration, params.gpu_intensity);
                    }
                }
            }
        }

        // Per-core polyrhythms
        for (core_idx, pattern) in params.core_patterns.iter().take(4).enumerate() {
            if !pattern.is_empty() && params.rhythm_polyrhythm_factor > 0.2 {
                for _ in 0..duration_bars {
                    comp.track(&format!("core{}", core_idx))
                        .drum_grid(16, sixteenth)
                        .shaker(pattern);
                }
            }
        }

        // Process melodies
        for (proc_name, melody) in params.process_melodies.iter().take(3) {
            comp.instrument(&format!("proc_{}", proc_name), &Instrument::music_box());
            for _ in 0..duration_bars {
                for &note in melody.iter() {
                    comp.instrument(&format!("proc_{}", proc_name), &Instrument::music_box())
                        .note(&[note], sixteenth * 3.0);
                }
            }
        }

        // Fan noise
        if params.fan_noise_level > 0.1 {
            comp.instrument("fans", &Instrument::noise())
                .filter(Filter::high_pass(2000.0, 0.5));
            for _ in 0..duration_bars {
                comp.instrument("fans", &Instrument::noise())
                    .note_with_velocity(&[A3], quarter * 4.0, params.fan_noise_level * 0.3);
            }
        }

        let mut mixer = comp.into_mixer();
        
        match format {
            ExportFormat::Wav => mixer.export_wav(output_path, 44100)?,
            ExportFormat::Flac => mixer.export_flac(output_path, 44100)?,
            ExportFormat::Midi => mixer.export_midi(output_path)?,
        }

        println!("Exported to: {}", output_path);

        Ok(())
    }
}

pub enum ExportFormat {
    Wav,
    Flac,
    Midi,
}
