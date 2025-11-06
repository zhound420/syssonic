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

        // === BASS (Memory Usage) ===
        // Deep, sustained bass notes that reflect memory pressure
        comp.instrument("bass", &Instrument::sub_bass())
            .filter(Filter::low_pass(800.0, 0.8))
            .effect(Effect::distortion(params.bass_velocity * 0.3));

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

        // === HI-HATS (Network Activity) ===
        // Subtle hi-hats that increase with network traffic
        let hihat_pattern: Vec<usize> = (0..16)
            .filter(|i| i % 2 == 0) // Every other 16th note
            .collect();
        
        for _ in 0..duration_bars {
            comp.track("hihats")
                .drum_grid(16, sixteenth)
                .hihat(&hihat_pattern);
        }

        // Play the composition
        let mixer = comp.into_mixer();
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

        comp.instrument("bass", &Instrument::sub_bass())
            .filter(Filter::low_pass(800.0, 0.8))
            .effect(Effect::distortion(params.bass_velocity * 0.3));

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

        let hihat_pattern: Vec<usize> = (0..16)
            .filter(|i| i % 2 == 0)
            .collect();
        
        for _ in 0..duration_bars {
            comp.track("hihats")
                .drum_grid(16, sixteenth)
                .hihat(&hihat_pattern);
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
