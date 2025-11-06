use crate::composer::{SystemComposer, ExportFormat};
use crate::mapper::MusicalParams;
use anyhow::Result;
use crossbeam_channel::{Sender, Receiver, bounded, unbounded};
use std::path::PathBuf;
use std::thread::{self, JoinHandle};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicF32, Ordering};

#[derive(Debug, Clone, serde::Serialize)]
pub enum AudioCommand {
    Play(MusicalParams, usize), // params, duration_bars
    Stop,
    Pause,
    Resume,
    SetVolume(f32),
    Export {
        path: PathBuf,
        format: String,
        params: MusicalParams,
        bars: usize,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AudioEvent {
    Playing,
    Stopped,
    Paused,
    Resumed,
    Error(String),
    ExportStarted,
    ExportProgress(f32),
    ExportComplete(String),
}

pub struct AudioThread {
    cmd_tx: Sender<AudioCommand>,
    event_rx: Receiver<AudioEvent>,
    thread_handle: Option<JoinHandle<()>>,
    is_playing: Arc<AtomicBool>,
    volume: Arc<AtomicF32>,
}

impl AudioThread {
    pub fn new() -> Self {
        let (cmd_tx, cmd_rx) = bounded::<AudioCommand>(32);
        let (event_tx, event_rx) = unbounded::<AudioEvent>();

        let is_playing = Arc::new(AtomicBool::new(false));
        let volume = Arc::new(AtomicF32::new(0.8));

        let is_playing_clone = is_playing.clone();
        let volume_clone = volume.clone();

        let thread_handle = thread::spawn(move || {
            // Audio thread main loop
            loop {
                match cmd_rx.recv() {
                    Ok(AudioCommand::Play(params, bars)) => {
                        is_playing_clone.store(true, Ordering::SeqCst);
                        let _ = event_tx.send(AudioEvent::Playing);

                        match SystemComposer::new() {
                            Ok(composer) => {
                                match composer.compose_and_play(&params, bars) {
                                    Ok(_) => {
                                        is_playing_clone.store(false, Ordering::SeqCst);
                                        let _ = event_tx.send(AudioEvent::Stopped);
                                    }
                                    Err(e) => {
                                        is_playing_clone.store(false, Ordering::SeqCst);
                                        let _ = event_tx.send(AudioEvent::Error(e.to_string()));
                                    }
                                }
                            }
                            Err(e) => {
                                is_playing_clone.store(false, Ordering::SeqCst);
                                let _ = event_tx.send(AudioEvent::Error(e.to_string()));
                            }
                        }
                    }

                    Ok(AudioCommand::Stop) => {
                        is_playing_clone.store(false, Ordering::SeqCst);
                        let _ = event_tx.send(AudioEvent::Stopped);
                        // TODO: Implement actual stop (tunes doesn't provide easy stop)
                    }

                    Ok(AudioCommand::Pause) => {
                        is_playing_clone.store(false, Ordering::SeqCst);
                        let _ = event_tx.send(AudioEvent::Paused);
                        // TODO: Implement pause
                    }

                    Ok(AudioCommand::Resume) => {
                        is_playing_clone.store(true, Ordering::SeqCst);
                        let _ = event_tx.send(AudioEvent::Resumed);
                        // TODO: Implement resume
                    }

                    Ok(AudioCommand::SetVolume(vol)) => {
                        volume_clone.store(vol, Ordering::SeqCst);
                        // TODO: Apply volume to audio engine
                    }

                    Ok(AudioCommand::Export { path, format, params, bars }) => {
                        let _ = event_tx.send(AudioEvent::ExportStarted);

                        let export_format = match format.to_lowercase().as_str() {
                            "wav" => ExportFormat::Wav,
                            "flac" => ExportFormat::Flac,
                            "midi" => ExportFormat::Midi,
                            _ => ExportFormat::Wav,
                        };

                        match SystemComposer::new() {
                            Ok(composer) => {
                                match composer.compose_and_export(&params, bars, path.to_str().unwrap(), export_format) {
                                    Ok(_) => {
                                        let _ = event_tx.send(AudioEvent::ExportComplete(path.to_string_lossy().to_string()));
                                    }
                                    Err(e) => {
                                        let _ = event_tx.send(AudioEvent::Error(e.to_string()));
                                    }
                                }
                            }
                            Err(e) => {
                                let _ = event_tx.send(AudioEvent::Error(e.to_string()));
                            }
                        }
                    }

                    Err(_) => break, // Channel closed, exit thread
                }
            }
        });

        AudioThread {
            cmd_tx,
            event_rx,
            thread_handle: Some(thread_handle),
            is_playing,
            volume,
        }
    }

    pub fn send_command(&self, cmd: AudioCommand) -> Result<()> {
        self.cmd_tx.send(cmd)?;
        Ok(())
    }

    pub fn poll_events(&self) -> Vec<AudioEvent> {
        self.event_rx.try_iter().collect()
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing.load(Ordering::SeqCst)
    }

    pub fn get_volume(&self) -> f32 {
        self.volume.load(Ordering::SeqCst)
    }
}

impl Drop for AudioThread {
    fn drop(&mut self) {
        // Send stop command and wait for thread
        let _ = self.cmd_tx.send(AudioCommand::Stop);
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}
