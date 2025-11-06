# SysSonic 🎵🖥️

**Turn your system metrics into music**

SysSonic is a real-time system monitoring tool that sonifies your computer's performance data. Watch your CPU usage become melody, your disk I/O transform into rhythm, and your temperature control the atmosphere.

## Features

- **Real-time sonification** - Listen to your system performance as music
- **Multiple export formats** - Save snapshots as WAV, FLAC, or MIDI
- **Intelligent mapping** - Musically coherent translations of system metrics
- **Low overhead** - Minimal impact on system performance
- **Cross-platform** - Works on Linux, macOS, and Windows

## Metric → Music Mapping

| System Metric | Musical Element | Description |
|---------------|----------------|-------------|
| **CPU Usage** (0-100%) | Melody Pitch | Higher CPU = higher notes in A minor pentatonic scale |
| **Memory Usage** (0-100%) | Bass Intensity | More memory used = deeper, more sustained bass |
| **Disk I/O** (read/write) | Percussion Density | Heavy I/O = complex rhythmic patterns |
| **Network Traffic** (bytes/sec) | Tempo | More traffic = faster tempo (90-130 BPM) |
| **Temperature** (°C) | Filter & Reverb | Hotter = more open filters and spacious reverb |

## Installation

### Prerequisites

**Linux (Debian/Ubuntu):**
```bash
sudo apt install libasound2-dev
```

**Linux (Fedora/RHEL):**
```bash
sudo dnf install alsa-lib-devel
```

**macOS/Windows:** No additional dependencies needed

### Build from source

```bash
cd syssonic
cargo build --release
```

The binary will be at `target/release/syssonic`

## Usage

### Live Sonification

Listen to your system in real-time:

```bash
# Play 4-bar compositions with 16-second intervals
cargo run --release -- live

# Custom settings: 8 bars, 20-second intervals, 5 iterations
cargo run --release -- live --bars 8 --interval 20 --count 5
```

### Export Snapshot

Capture current system state as audio:

```bash
# Export as WAV (default)
cargo run --release -- export --output system_snapshot.wav

# Export as FLAC (compressed, lossless)
cargo run --release -- export --output system.flac --format flac

# Export as MIDI
cargo run --release -- export --output system.mid --format midi

# Longer composition with more averaging
cargo run --release -- export --output detailed.wav --bars 16 --samples 10
```

### Monitor Mode

See the mapping without audio (useful for debugging):

```bash
# Monitor every 2 seconds
cargo run --release -- monitor

# Custom interval and count
cargo run --release -- monitor --interval 5 --count 10
```

### Test Audio

Verify your audio setup:

```bash
cargo run --release -- test
```

## Musical Design Philosophy

SysSonic is designed to be **musically informative** rather than just data-to-sound. The mapping choices create coherent, listenable compositions that reflect system behavior:

- **A Minor Pentatonic Scale**: Inherently pleasant, avoids dissonance
- **Tempo Range (90-130 BPM)**: Human-comfortable, clearly perceivable
- **Layered Approach**: Each metric gets its own sonic space
  - Melody (high frequencies) = CPU
  - Bass (low frequencies) = Memory  
  - Drums (transients) = I/O
  - Atmosphere (textures) = Temperature

## Example Scenarios

**Idle System**
- Low, sparse melody (low CPU)
- Gentle bass (moderate memory)
- Minimal percussion (little I/O)
- Dry, tight sound (normal temperature)

**Under Load (compilation, video rendering)**
- High, busy melody (CPU spikes)
- Deep, sustained bass (high memory)
- Dense, complex rhythms (heavy disk activity)
- Open, spacious sound (increased temperature)

**Network Activity (downloading, streaming)**
- Faster tempo increase
- Steady percussion patterns
- Melodic variations

## Advanced Usage

### Generate a "Performance Profile"

Capture your system during different workloads:

```bash
# Idle state
cargo run --release -- export --output idle.wav --samples 20

# Under load (run your workload then...)
cargo run --release -- export --output under_load.wav --samples 20

# Compare the audio files to hear the difference!
```

### Long-form Monitoring

Create an extended composition showing system behavior over time:

```bash
cargo run --release -- live --bars 8 --interval 60 --count 60
# Records 1 hour of system performance (60 samples, 1 minute apart)
```

## Technical Details

- **Audio Engine**: 44.1kHz sample rate, 4096 sample buffer
- **Smoothing**: Multiple samples averaged to prevent erratic changes
- **Latency**: ~93ms (suitable for monitoring, not live performance)
- **CPU Impact**: < 5% on modern systems

## Extending SysSonic

Want to customize the mappings or add new metrics?

1. **Add new metrics** - Edit `src/metrics.rs`
2. **Change mappings** - Modify `src/mapper.rs`
3. **Adjust composition** - Update `src/composer.rs`
4. **Different scales** - Try Dorian, Phrygian, or other modes

## Future Ideas

- [ ] Web dashboard with real-time visualization
- [ ] Remote monitoring (sonify servers over SSH)
- [ ] Machine learning for anomaly detection via audio
- [ ] GPU metrics integration
- [ ] Docker/container metrics
- [ ] Proxmox cluster monitoring
- [ ] Custom scale/mode selection via CLI
- [ ] MIDI controller input for parameter tweaking

## License

MIT or Apache-2.0 (same as the `tunes` library)

## Credits

Built with [tunes](https://github.com/sqrew/tunes) - A Rust library for music composition and synthesis

---

**"Your infrastructure has never sounded so good!"** 🎧
