# SysSonic 🎵🖥️

**Turn your system metrics into music**

SysSonic is a real-time system monitoring tool that sonifies your computer's performance data. Watch your CPU usage become melody, your GPU activity add harmonic voices, your disk I/O transform into rhythm, and your temperature control the atmosphere.

## Two Ways to Use SysSonic

### 🖥️ System Tray Application (Recommended)
A polished desktop application with real-time dashboard, system tray integration, and full control suite.

**Features:**
- Lives in your system tray on macOS, Windows, or Linux
- Real-time visual dashboard with charts and metrics
- Per-core CPU monitoring
- GPU metrics (NVIDIA/AMD)
- Battery monitoring
- Audio controls (start/stop/pause/volume)
- Export snapshots as WAV/FLAC/MIDI
- Persistent settings

👉 **[See tray/README.md for installation instructions](tray/README.md)**

### ⌨️ Command Line Interface
A lightweight CLI tool for quick monitoring, scripting, and headless servers.

**Use cases:**
- Quick system sonification without GUI
- Server monitoring over SSH
- Automated performance profiling
- Integration with shell scripts

👉 **[See CLI usage below](#cli-usage)**

---

## Features

- ✅ **Real-time sonification** - Listen to your system performance as music
- ✅ **Extended metrics monitoring** - CPU (per-core), GPU, memory, battery, disk, network, temperature, processes
- ✅ **System tray application** - Beautiful desktop GUI with dashboard
- ✅ **Multiple export formats** - Save snapshots as WAV, FLAC, or MIDI
- ✅ **Intelligent mapping** - Musically coherent translations of system metrics
- ✅ **Low overhead** - Minimal impact on system performance (< 5% CPU)
- ✅ **Cross-platform** - Works on Linux, macOS, and Windows

## Extended Metric → Music Mapping

| System Metric | Musical Element | Description |
|---------------|----------------|-------------|
| **CPU Usage** (0-100%) | Melody Pitch | Higher CPU = higher notes in A minor pentatonic scale |
| **GPU Usage** (0-100%) | Secondary Voice | GPU activity = Dorian mode melody with chorus/flanger effects |
| **Memory Usage** (0-100%) | Bass Intensity | More memory used = deeper, more sustained bass |
| **Swap Usage** (0-100%) | Bass Distortion | High swap = distorted bass (0-100% distortion) |
| **VRAM Usage** (0-100%) | Reverb Size | More VRAM used = larger reverb space |
| **Disk I/O** (read/write) | Percussion Density | Heavy I/O = complex rhythmic patterns |
| **Network Traffic** (bytes/sec) | Tempo | More traffic = faster tempo (90-130 BPM) |
| **Temperature** (°C) | Filter & Reverb | Hotter = more open filters and spacious reverb |
| **Load Average** (1/5/15 min) | Polyrhythm Factor | Higher load = more complex polyrhythmic patterns |
| **Per-Core CPU** | Shaker Patterns | Each core drives independent rhythmic shaker patterns |
| **Process Count** | Hi-Hat Density | More processes = denser hi-hat patterns |
| **Top Processes** | Mini-Melodies | Top 3 CPU-heavy processes = music box melodies |
| **Battery Level** (%) | Volume & Tonality | Low battery = quieter, darker tones |
| **Fan Speeds** (RPM) | Noise Level | Higher RPM = filtered white noise layer |

---

## CLI Usage

### Prerequisites

**Rust 1.70+** (install from [rustup.rs](https://rustup.rs))

**Linux (Debian/Ubuntu):**
```bash
sudo apt install libasound2-dev build-essential
```

**Linux (Fedora/RHEL):**
```bash
sudo dnf install alsa-lib-devel
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
- No additional dependencies needed

### Build from Source

```bash
# Clone the repository
git clone https://github.com/zhound420/syssonic.git
cd syssonic

# Build the CLI
cargo build --release

# The binary will be at target/release/syssonic
```

### Live Sonification

Listen to your system in real-time:

```bash
# Play 4-bar compositions with 16-second intervals
./target/release/syssonic live

# Custom settings: 8 bars, 20-second intervals, 5 iterations
./target/release/syssonic live --bars 8 --interval 20 --count 5

# Continuous monitoring (Ctrl+C to stop)
./target/release/syssonic live --count 999
```

### Export Snapshot

Capture current system state as audio:

```bash
# Export as WAV (default)
./target/release/syssonic export --output system_snapshot.wav

# Export as FLAC (compressed, lossless)
./target/release/syssonic export --output system.flac --format flac

# Export as MIDI
./target/release/syssonic export --output system.mid --format midi

# Longer composition with more averaging (smoother)
./target/release/syssonic export --output detailed.wav --bars 16 --samples 10
```

### Monitor Mode

See the metric → music mapping without audio (useful for debugging):

```bash
# Monitor every 2 seconds
./target/release/syssonic monitor

# Custom interval and count
./target/release/syssonic monitor --interval 5 --count 10
```

### Test Audio

Verify your audio setup:

```bash
./target/release/syssonic test
```

---

## Musical Design Philosophy

SysSonic is designed to be **musically informative** rather than just data-to-sound. The mapping choices create coherent, listenable compositions that reflect system behavior:

### Core Principles

- **A Minor Pentatonic Scale**: Inherently pleasant, avoids dissonance
- **Dorian Mode (GPU)**: Adds harmonic interest without clashing
- **Tempo Range (90-130 BPM)**: Human-comfortable, clearly perceivable
- **Layered Approach**: Each metric gets its own sonic space
  - **Melody** (high frequencies) = CPU usage
  - **Secondary Voice** (mid-high) = GPU activity
  - **Bass** (low frequencies) = Memory + Swap
  - **Drums** (transients) = Disk I/O + Per-core patterns
  - **Hi-hats** (high transients) = Process count
  - **Music Box** (delicate) = Top processes
  - **Atmosphere** (textures) = Temperature + Reverb
  - **Noise** (white noise) = Fan speeds

### Sonic Characteristics by System State

**Idle System:**
- Low, sparse melody (low CPU)
- Minimal GPU voice (or silent)
- Gentle bass (moderate memory)
- Minimal percussion (little I/O)
- Simple rhythms (few cores active)
- Dry, tight sound (normal temperature)

**Under Heavy Load (compilation, video rendering, gaming):**
- High, busy melody (CPU spikes)
- Prominent GPU voice with effects
- Deep, sustained bass (high memory)
- Possibly distorted bass (swap usage)
- Dense, complex rhythms (heavy disk activity)
- Polyrhythmic shaker patterns (per-core activity)
- Dense hi-hats (many processes)
- Open, spacious sound (increased temperature)
- Fan noise layer (cooling system active)

**Network Activity (downloading, streaming):**
- Faster tempo increase (90 → 130 BPM)
- Steady percussion patterns
- Melodic variations based on bandwidth

**Low Battery (laptops):**
- Quieter overall volume
- Darker, more muted tonality
- Sense of urgency in the musical character

---

## Example Use Cases

### Performance Profiling

Capture your system during different workloads:

```bash
# Baseline (idle)
./target/release/syssonic export --output idle.wav --samples 20

# Under load (start your workload, then run)
./target/release/syssonic export --output compile.wav --samples 20

# Compare the audio files to hear the difference!
```

### Long-form Monitoring

Create an extended composition showing system behavior over time:

```bash
# Record 1 hour of system performance (60 samples, 1 minute apart)
./target/release/syssonic live --bars 8 --interval 60 --count 60
```

### Server Monitoring

Monitor a remote server over SSH:

```bash
ssh user@server 'cd syssonic && ./target/release/syssonic monitor --interval 10 --count 6'
```

### Integration with Scripts

Use in shell scripts for automated monitoring:

```bash
#!/bin/bash
# Monitor system during a build
./target/release/syssonic live --bars 4 --count 1 &
AUDIO_PID=$!

# Run your workload
make clean && make -j$(nproc)

# Stop monitoring
kill $AUDIO_PID
```

---

## Technical Details

### Audio Engine
- **Sample Rate**: 44.1kHz
- **Buffer Size**: 4096 samples
- **Latency**: ~93ms (suitable for monitoring, not live performance)
- **CPU Impact**: < 5% on modern systems

### Metrics Collection
- **Update Frequency**: Configurable (default: 2 seconds in GUI, 16 seconds in CLI)
- **Smoothing**: Multiple samples averaged to prevent erratic changes
- **Graceful Degradation**: Optional hardware (GPU, battery, fans) handled cleanly
- **Platform Support**:
  - CPU/Memory/Disk/Network: All platforms
  - GPU NVIDIA: Linux, Windows (via NVML)
  - GPU AMD: Linux (via hwmon)
  - Battery: All platforms with batteries
  - Fan speeds: Linux only (via lm-sensors)

### Audio Synthesis
- **Synthesis Engine**: [tunes](https://github.com/sqrew/tunes) 0.5.0
- **Voices**:
  - Sine wave oscillators (melody)
  - Sawtooth oscillators (bass)
  - Analog synth (GPU voice)
  - 808-style drums (kick, snare, hi-hat)
  - Shaker samples (per-core rhythms)
  - Music box (process melodies)
  - White noise (fan speeds)
- **Effects**: Low-pass filter, reverb, distortion, chorus, flanger

---

## Project Structure

```
syssonic/
├── src/                    # CLI source code
│   ├── main.rs            # CLI entry point and argument parsing
│   ├── metrics/           # System metrics collection
│   │   ├── mod.rs         # Module exports
│   │   ├── system.rs      # Core metrics (CPU, memory, disk, network)
│   │   ├── gpu_nvidia.rs  # NVIDIA GPU monitoring
│   │   ├── gpu_amd.rs     # AMD GPU monitoring
│   │   ├── battery.rs     # Battery monitoring
│   │   ├── fans.rs        # Fan speed monitoring
│   │   └── processes.rs   # Process list and top consumers
│   ├── mapper.rs          # Metrics → Musical parameter mapping
│   └── composer.rs        # Audio composition and playback
├── tray/                  # System tray GUI application
│   ├── src/               # React frontend
│   │   ├── App.tsx        # Main dashboard component
│   │   └── App.css        # Styling
│   ├── src-tauri/         # Rust backend
│   │   └── src/
│   │       ├── lib.rs     # Tauri setup & tray menu
│   │       ├── commands.rs # IPC command handlers
│   │       ├── audio_thread.rs # Audio playback thread
│   │       ├── config.rs  # Configuration management
│   │       └── metrics/   # Shared metrics code
│   └── README.md          # GUI installation guide
├── Cargo.toml             # Rust dependencies
├── CLAUDE.md              # AI assistant context file
└── README.md              # This file
```

---

## Extending SysSonic

### Adding New Metrics

1. **Collect the metric** in `src/metrics/system.rs`:
```rust
pub struct SystemMetrics {
    // ... existing fields
    pub my_new_metric: f32,
}
```

2. **Map to musical parameter** in `src/mapper.rs`:
```rust
impl MetricsMapper {
    pub fn map_my_metric(&self, value: f32) -> f32 {
        // Transform metric to musical range (e.g., 0-1, frequency, etc.)
    }
}
```

3. **Use in composition** in `src/composer.rs`:
```rust
let my_param = self.mapper.map_my_metric(metrics.my_new_metric);
// Apply to synthesis (volume, pitch, effect, etc.)
```

### Customizing Musical Mappings

Edit thresholds and ranges in `src/mapper.rs`:

```rust
const CPU_LOW: f32 = 20.0;      // CPU below 20% = low activity
const CPU_HIGH: f32 = 80.0;     // CPU above 80% = high activity
const TEMPO_MIN: f32 = 90.0;    // Minimum tempo (BPM)
const TEMPO_MAX: f32 = 130.0;   // Maximum tempo (BPM)
```

### Using Different Scales

Modify the scale in `src/mapper.rs`:

```rust
// Current: A minor pentatonic [A, C, D, E, G]
const PENTATONIC_SCALE: [f32; 5] = [220.0, 261.63, 293.66, 329.63, 392.0];

// Example: Major pentatonic [C, D, E, G, A]
const PENTATONIC_SCALE: [f32; 5] = [261.63, 293.66, 329.63, 392.0, 440.0];
```

---

## Troubleshooting

### "Failed to initialize GPU metrics"
- **Cause**: GPU drivers not installed or GPU not detected
- **Solution**: Install NVIDIA/AMD drivers, or disable GPU monitoring
- **Note**: This is non-fatal; other metrics will still work

### "Audio device not found" or no sound
- **Linux**: Install ALSA (`libasound2-dev` or `alsa-lib-devel`)
- **All platforms**: Check system audio is working with other applications
- **Advanced**: Try different audio backend in `composer.rs`

### High CPU usage
- **GUI**: Increase `update_interval_ms` in settings (default: 16000 = 16s)
- **CLI**: Use longer `--interval` (e.g., `--interval 30`)
- **Disable**: Turn off GPU monitoring if not needed

### Build errors

**Missing dependencies:**
```bash
# Linux: Install ALSA development libraries
sudo apt install libasound2-dev build-essential pkg-config

# Also install GPU monitoring dependencies if needed
sudo apt install libvulkan-dev  # For AMD GPU support
```

**Rust version too old:**
```bash
rustup update stable
```

**tunes crate errors:**
- Ensure you're building in release mode: `cargo build --release`
- Check that audio dependencies are installed

---

## Future Ideas

- [x] ~~GPU metrics integration~~ ✅ Implemented (NVIDIA + AMD)
- [x] ~~System tray application~~ ✅ Implemented (Tauri-based)
- [ ] Web dashboard with real-time visualization
- [ ] Remote monitoring (sonify servers over SSH)
- [ ] Machine learning for anomaly detection via audio
- [ ] Docker/container metrics
- [ ] Proxmox cluster monitoring
- [ ] Custom scale/mode selection via CLI
- [ ] MIDI controller input for parameter tweaking
- [ ] 3D visualization in tray app
- [ ] Real-time chart overlays in tray app
- [ ] Streaming mode (broadcast to network)
- [ ] Alert system (audio signatures for anomalies)

---

## Contributing

Contributions are welcome! Here are some ways to help:

1. **Test on different hardware**: Especially GPU monitoring (NVIDIA/AMD), battery monitoring, fan sensors
2. **Add new metrics**: System load, network latency, process-specific monitoring
3. **Improve musical mappings**: Better scales, more interesting synthesis
4. **Cross-platform testing**: Ensure tray app works on Windows/macOS/Linux
5. **Documentation**: Improve examples, add tutorials, record demo videos

### Development Setup

```bash
# CLI development
cargo build
cargo run -- live

# GUI development
cd tray
npm install
npm run tauri dev
```

See [CLAUDE.md](CLAUDE.md) for detailed architecture documentation.

---

## License

MIT or Apache-2.0 (your choice, same as the `tunes` library)

## Credits

Built with:
- [tunes](https://github.com/sqrew/tunes) - Rust library for music composition and synthesis
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - Cross-platform system information
- [nvml-wrapper](https://github.com/Cldfire/nvml-wrapper) - NVIDIA GPU monitoring
- [libamdgpu_top](https://github.com/Umio-Yasuno/libamdgpu_top) - AMD GPU monitoring
- [battery](https://github.com/svartalf/rust-battery) - Battery information
- [Tauri](https://tauri.app/) - Desktop application framework (tray app)

---

**"Your infrastructure has never sounded so good!"** 🎧

*Listen to your system, not just monitor it.*
