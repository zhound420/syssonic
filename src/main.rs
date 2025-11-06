mod metrics;
mod mapper;
mod composer;

use metrics::MetricsCollector;
use mapper::MetricsMapper;
use composer::{SystemComposer, ExportFormat};
use clap::{Parser, Subcommand};
use anyhow::Result;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "syssonic")]
#[command(about = "Turn your system metrics into music", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Play live system sonification (real-time monitoring)
    Live {
        /// Number of bars per sample (default: 4)
        #[arg(short, long, default_value_t = 4)]
        bars: usize,

        /// Seconds between updates (default: 16)
        #[arg(short, long, default_value_t = 16.0)]
        interval: f32,

        /// Number of iterations (0 = infinite)
        #[arg(short, long, default_value_t = 0)]
        count: usize,
    },

    /// Capture a snapshot and export to file
    Export {
        /// Output file path
        #[arg(short, long)]
        output: String,

        /// Export format: wav, flac, or midi
        #[arg(short, long, default_value = "wav")]
        format: String,

        /// Number of bars to generate (default: 8)
        #[arg(short, long, default_value_t = 8)]
        bars: usize,

        /// Number of samples to average (default: 5)
        #[arg(short, long, default_value_t = 5)]
        samples: usize,
    },

    /// Show current system metrics (no audio)
    Monitor {
        /// Update interval in seconds
        #[arg(short, long, default_value_t = 2.0)]
        interval: f32,

        /// Number of iterations (0 = infinite)
        #[arg(short, long, default_value_t = 0)]
        count: usize,
    },

    /// Test audio setup with a simple composition
    Test,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Live { bars, interval, count } => {
            live_sonification(bars, interval, count)?;
        }
        Commands::Export { output, format, bars, samples } => {
            export_snapshot(&output, &format, bars, samples)?;
        }
        Commands::Monitor { interval, count } => {
            monitor_metrics(interval, count)?;
        }
        Commands::Test => {
            test_audio()?;
        }
    }

    Ok(())
}

fn live_sonification(bars: usize, interval_secs: f32, count: usize) -> Result<()> {
    println!("🎵 SysSonic - Live System Sonification");
    println!("Press Ctrl+C to stop\n");

    let mut collector = MetricsCollector::new();
    let mapper = MetricsMapper::new();
    let composer = SystemComposer::new()?;

    let mut iteration = 0;
    loop {
        if count > 0 && iteration >= count {
            break;
        }

        println!("🔄 Collecting metrics...");
        let metrics = collector.collect_smoothed(3, 200);
        let params = mapper.map(&metrics);
        
        mapper.print_mapping_info(&metrics, &params);

        println!("🎹 Playing composition ({} bars)...", bars);
        composer.compose_and_play(&params, bars)?;

        if count > 0 {
            iteration += 1;
            if iteration < count {
                println!("\n⏸  Waiting {:.1}s before next sample...\n", interval_secs);
                std::thread::sleep(Duration::from_secs_f32(interval_secs));
            }
        } else {
            println!("\n⏸  Waiting {:.1}s before next sample...\n", interval_secs);
            std::thread::sleep(Duration::from_secs_f32(interval_secs));
        }
    }

    println!("\n✅ Live sonification complete!");
    Ok(())
}

fn export_snapshot(output: &str, format_str: &str, bars: usize, samples: usize) -> Result<()> {
    println!("🎵 SysSonic - Export Snapshot");
    println!("📊 Collecting {} samples...", samples);

    let mut collector = MetricsCollector::new();
    let mapper = MetricsMapper::new();
    let composer = SystemComposer::new()?;

    let metrics = collector.collect_smoothed(samples, 200);
    let params = mapper.map(&metrics);
    
    mapper.print_mapping_info(&metrics, &params);

    let format = match format_str.to_lowercase().as_str() {
        "wav" => ExportFormat::Wav,
        "flac" => ExportFormat::Flac,
        "midi" => ExportFormat::Midi,
        _ => {
            eprintln!("❌ Unknown format '{}'. Using WAV.", format_str);
            ExportFormat::Wav
        }
    };

    println!("🎹 Generating composition ({} bars)...", bars);
    composer.compose_and_export(&params, bars, output, format)?;

    println!("✅ Export complete!");
    Ok(())
}

fn monitor_metrics(interval_secs: f32, count: usize) -> Result<()> {
    println!("📊 SysSonic - Metrics Monitor");
    println!("Press Ctrl+C to stop\n");

    let mut collector = MetricsCollector::new();
    let mapper = MetricsMapper::new();

    let mut iteration = 0;
    loop {
        if count > 0 && iteration >= count {
            break;
        }

        let metrics = collector.collect();
        let params = mapper.map(&metrics);
        
        mapper.print_mapping_info(&metrics, &params);

        if count > 0 {
            iteration += 1;
            if iteration < count {
                std::thread::sleep(Duration::from_secs_f32(interval_secs));
            }
        } else {
            std::thread::sleep(Duration::from_secs_f32(interval_secs));
        }
    }

    println!("\n✅ Monitoring complete!");
    Ok(())
}

fn test_audio() -> Result<()> {
    println!("🎵 SysSonic - Audio Test");
    println!("Playing test composition...\n");

    use tunes::prelude::*;

    let engine = AudioEngine::new()?;
    let mut comp = Composition::new(Tempo::new(120.0));

    // Simple test melody
    comp.instrument("test", &Instrument::synth_lead())
        .notes(&[C4, E4, G4, C5], 0.5);

    comp.track("drums")
        .drum_grid(16, 0.125)
        .kick(&[0, 4, 8, 12])
        .snare(&[4, 12]);

    let mixer = comp.into_mixer();
    engine.play_mixer(&mixer)?;

    println!("✅ Test complete! If you heard sound, audio is working.");
    Ok(())
}
