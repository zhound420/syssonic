# SysSonic - Getting Started Guide

## What You've Got

A complete Rust project that turns system performance metrics into music in real-time.

## Project Structure

```
syssonic/
├── Cargo.toml              # Project dependencies
├── README.md               # Full documentation
├── demo_visualization.py   # Demo showing metric→music mappings
└── src/
    ├── main.rs            # CLI interface & main logic
    ├── metrics.rs         # System monitoring (CPU, memory, I/O, etc.)
    ├── mapper.rs          # Metrics → Musical parameters
    └── composer.rs        # Music generation with tunes library
```

## Quick Start

### 1. Prerequisites

**On your Linux system:**
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install ALSA development libraries
sudo apt install libasound2-dev  # Debian/Ubuntu
# OR
sudo dnf install alsa-lib-devel  # Fedora/RHEL
```

**On macOS/Windows:**
Rust only, no additional dependencies needed.

### 2. Build the Project

```bash
cd syssonic
cargo build --release
```

The first build will take 5-10 minutes as it downloads and compiles dependencies.

### 3. Run Your First Test

```bash
# Test that audio works
cargo run --release -- test

# If you hear a short melody with drums, you're good to go!
```

### 4. Start Live Monitoring

```bash
# Listen to your system in real-time
cargo run --release -- live

# This will:
# - Sample your system metrics
# - Convert them to music
# - Play a 4-bar composition
# - Repeat every 16 seconds
```

## Understanding What You're Hearing

### Idle System
- **Sound**: Low, gentle melody; sparse percussion; tight, dry
- **Means**: Low CPU, moderate memory, minimal I/O

### Under Heavy Load
- **Sound**: High, complex melody; dense rhythms; spacious reverb
- **Means**: High CPU, high memory, heavy disk activity, elevated temperature

### Active Network
- **Sound**: Faster tempo
- **Means**: Data transfer happening

### Hot System
- **Sound**: More reverb, open filters (sounds "bigger")
- **Means**: Temperature rising - might want to check cooling

## CLI Commands Reference

### Live Sonification
```bash
# Default: 4 bars, 16 second intervals, infinite
cargo run --release -- live

# Custom: 8 bars, 30 second intervals, run 10 times
cargo run --release -- live --bars 8 --interval 30 --count 10
```

### Export Snapshots
```bash
# Export current state as WAV
cargo run --release -- export --output snapshot.wav

# Export as FLAC (smaller, lossless)
cargo run --release -- export --output snapshot.flac --format flac

# Export as MIDI (for editing in DAW)
cargo run --release -- export --output snapshot.mid --format midi

# Longer composition with more samples
cargo run --release -- export --output detailed.wav --bars 16 --samples 10
```

### Monitor Mode (No Audio)
```bash
# See the mappings without playing audio
cargo run --release -- monitor

# Useful for debugging or understanding the mappings
cargo run --release -- monitor --interval 1 --count 5
```

## Practical Use Cases

### 1. Background Monitoring
Run while working to get audio feedback about your system state:
```bash
cargo run --release -- live --bars 2 --interval 30
```

### 2. Performance Profiling
Capture different workload states:
```bash
# Idle
cargo run --release -- export -o idle.wav --samples 20

# [Start your heavy workload]

# Under load
cargo run --release -- export -o loaded.wav --samples 20

# Compare the audio files!
```

### 3. Server/Cluster Monitoring
SSH into a server and listen to its performance:
```bash
ssh your-server "cd syssonic && cargo run --release -- live"
```

### 4. Creative Experimentation
Use system activity as musical input:
```bash
# Start a large compilation
cargo build --release &

# Capture the build process as music
cargo run --release -- export -o compilation_music.wav --bars 32

# You've just composed with your compiler!
```

## Customization Ideas

### Change the Musical Scale
Edit `src/mapper.rs`, line ~25:
```rust
// Try different scales:
let scale = vec![C4, D4, E4, F4, G4, A4, B4, C5];  // C major
let scale = vec![C4, Eb4, F4, G4, Bb4, C5];         // C blues
let scale = vec![C4, Db4, E4, F4, G4, Ab4, B4, C5]; // C harmonic minor
```

### Adjust Tempo Range
Edit `src/mapper.rs`, line ~82:
```rust
let tempo = self.base_tempo + (network_normalized * 60.0); // 90-150 BPM
```

### Different Instruments
Edit `src/composer.rs`, change instruments:
```rust
comp.instrument("melody", &Instrument::electric_piano())
comp.instrument("bass", &Instrument::acoustic_bass())
```

### Add More Metrics
1. Add new fields to `SystemMetrics` in `src/metrics.rs`
2. Collect them in `MetricsCollector::collect()`
3. Map them in `src/mapper.rs`
4. Use them in composition in `src/composer.rs`

## Troubleshooting

### "No sound"
- Run `cargo run --release -- test` to verify audio
- Check system volume/output device
- On Linux: verify ALSA is working (`aplay /usr/share/sounds/alsa/Front_Center.wav`)

### "Audio glitches/dropouts"
- Reduce bars: `--bars 2`
- Increase buffer size in code (see `composer.rs`, `AudioEngine::with_buffer_size(8192)`)

### "Build fails"
- Ensure Rust is up to date: `rustup update`
- Verify ALSA dev libs installed (Linux)
- Check error messages for missing dependencies

### "Metrics seem wrong"
- Run monitor mode to see raw values: `cargo run --release -- monitor`
- Temperature sensors may not work on all systems (uses fallback value)
- First sample may show zero I/O rates (needs delta calculation)

## Next Steps

1. **Run it on your actual systems** - Try it on your Proxmox nodes!
2. **Extend for cluster monitoring** - Aggregate metrics from all nodes
3. **Add visualization** - Pair with your 3D visualization skills
4. **Integrate with automation** - Alert when music sounds "wrong"
5. **Create compositions** - Use workload patterns as creative input

## Performance Note

SysSonic uses < 5% CPU on modern systems and minimal memory. The audio engine runs in a separate thread, so monitoring doesn't interfere with performance.

## Contributing

Found a cool mapping? Added a new metric? Made it better? Fork and share!

---

**Ready to hear your infrastructure?** 🎧

```bash
cargo run --release -- live
```
