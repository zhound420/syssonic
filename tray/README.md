# SysSonic System Tray Application

A cross-platform system tray application for SysSonic that provides real-time system monitoring and audio sonification controls.

## Features

- **System Tray Integration**: Lives in your system tray with quick access to all controls
- **Real-time Dashboard**: Monitor CPU, memory, GPU, battery, and more with live updates
- **Audio Controls**: Start/stop sonification, adjust volume, export snapshots
- **Per-Core CPU Monitoring**: Visualize each CPU core's usage
- **GPU Metrics**: NVIDIA and AMD GPU support with temperature, utilization, VRAM usage
- **Battery Monitoring**: State of charge, power draw, time estimates
- **Musical Parameters**: See how your system metrics map to musical elements
- **Persistent Settings**: Configuration saved automatically

## Architecture

### Backend (Rust)
- **Audio Thread**: Non-blocking audio playback using crossbeam channels
- **Metrics Collection**: Extended metrics from parent SysSonic project
- **Config Management**: TOML-based persistence with platform-specific directories
- **IPC Commands**: 14 Tauri commands for frontend communication
- **Single Instance**: Prevents multiple app instances

### Frontend (React + TypeScript)
- **Dashboard**: Real-time metrics visualization with 2-second polling
- **Settings Page**: Configure theme, auto-start, monitoring options
- **Responsive UI**: Dark theme with smooth animations

## Prerequisites

### All Platforms
- Node.js 18+ and npm
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))

### Platform-Specific

**Linux (Debian/Ubuntu):**
```bash
sudo apt install libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libasound2-dev
```

**Linux (Fedora/RHEL):**
```bash
sudo dnf install webkit2gtk4.0-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel \
  alsa-lib-devel
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
- Microsoft Visual Studio C++ Build Tools
- WebView2 (usually pre-installed on Windows 11)

## Installation

### 1. Install Dependencies

```bash
npm install
```

### 2. Build the Application

**Development mode (hot reload):**
```bash
npm run tauri dev
```

**Production build:**
```bash
npm run tauri build
```

The production binary will be located at:
- **Linux**: `src-tauri/target/release/syssonic-tray`
- **macOS**: `src-tauri/target/release/bundle/macos/syssonic-tray.app`
- **Windows**: `src-tauri/target/release/syssonic-tray.exe`

### 3. Install System-Wide (Optional)

**Linux:**
```bash
sudo cp src-tauri/target/release/syssonic-tray /usr/local/bin/
```

**macOS:**
```bash
cp -r src-tauri/target/release/bundle/macos/syssonic-tray.app /Applications/
```

**Windows:**
Install using the generated `.msi` installer in `src-tauri/target/release/bundle/msi/`

## Usage

### Starting the App

Simply run the executable. The app will appear in your system tray.

**Tray Menu:**
- Show/Hide Window - Toggle main window visibility
- ▶ Start Sonification - Begin audio playback
- ⏹ Stop - Stop audio playback
- Volume - Submenu with 25%, 50%, 75%, 100% options
- 💾 Export Snapshot - Save current metrics as audio file
- ⚙ Settings - Open settings page
- ❌ Quit - Exit application

### Dashboard

Click the tray icon or use "Show/Hide Window" to access the full dashboard:

- **Audio Controls**: Start/stop/pause/resume with volume slider
- **System Metrics**: CPU, memory, temperature, load average, disk I/O, network
- **Per-Core CPU**: Grid showing each core's usage
- **GPU Metrics**: Utilization, temperature, power, VRAM (if available)
- **Battery Status**: Charge level, state, time estimates (laptops only)
- **Top Processes**: 5 most CPU-intensive processes
- **Musical Parameters**: Real-time mapping visualization

### Settings

Configure the application behavior:
- **Theme**: Dark/Light mode
- **Auto-start**: Launch on system boot
- **GPU Monitoring**: Enable/disable GPU metrics collection
- **Update Interval**: Metrics polling frequency (ms)

## Configuration

Config file location:
- **Linux**: `~/.config/syssonic/SysSonic/config.toml`
- **macOS**: `~/Library/Application Support/com.syssonic.SysSonic/config.toml`
- **Windows**: `%APPDATA%\syssonic\SysSonic\config\config.toml`

Example configuration:
```toml
audio_device = "default"
volume = 0.8
auto_play_on_start = false
update_interval_ms = 16000
sample_count = 3
base_tempo = 90.0
scale_type = "minor_pentatonic"
theme = "dark"
start_minimized = false
show_3d_viz = true
auto_start = false
enable_gpu_monitoring = true
enable_battery_monitoring = true
enable_fan_monitoring = true
```

## Development

### Project Structure

```
tray/
├── src/                    # React frontend
│   ├── App.tsx            # Main dashboard component
│   ├── App.css            # Styling
│   └── main.tsx           # Entry point
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── lib.rs         # Tauri setup & tray menu
│   │   ├── commands.rs    # IPC command handlers
│   │   ├── audio_thread.rs # Audio playback thread
│   │   ├── config.rs      # Configuration management
│   │   ├── composer.rs    # Audio composition
│   │   ├── mapper.rs      # Metrics → Music mapping
│   │   └── metrics/       # System metrics collection
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── package.json           # Node.js dependencies
└── vite.config.ts         # Vite configuration
```

### Adding New Tauri Commands

1. Add command function in `src-tauri/src/commands.rs`:
```rust
#[tauri::command]
pub fn my_command(state: State<AppState>) -> Result<String, String> {
    // Implementation
    Ok("Success".to_string())
}
```

2. Register in `src-tauri/src/lib.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    commands::my_command,
    // ... other commands
])
```

3. Call from frontend:
```typescript
import { invoke } from "@tauri-apps/api/core";
const result = await invoke<string>("my_command");
```

### Debugging

**Backend (Rust):**
```bash
npm run tauri dev
# Check terminal for Rust println!/eprintln! output
```

**Frontend (React):**
- Open DevTools in the Tauri window (Cmd/Ctrl+Shift+I)
- Check console.log/console.error output

## Building for Distribution

### Create Installers

```bash
npm run tauri build
```

This generates platform-specific installers:
- **Linux**: `.deb`, `.AppImage` in `src-tauri/target/release/bundle/`
- **macOS**: `.dmg`, `.app` in `src-tauri/target/release/bundle/`
- **Windows**: `.msi`, `.exe` in `src-tauri/target/release/bundle/`

### Code Signing (macOS/Windows)

For distribution outside of development, you'll need to sign the app:

**macOS:**
```bash
export APPLE_CERTIFICATE=...
export APPLE_CERTIFICATE_PASSWORD=...
export APPLE_SIGNING_IDENTITY=...
npm run tauri build
```

**Windows:**
- Obtain a code signing certificate
- Configure in `tauri.conf.json`

## Troubleshooting

### "Failed to initialize GPU metrics"
- Ensure NVIDIA/AMD drivers are installed
- GPU monitoring can be disabled in settings

### "Audio device not found"
- Check system audio is working
- Try selecting a different audio device in config.toml

### App won't start
- Check logs in terminal/console
- Verify all dependencies are installed
- Try deleting config file and restarting

### High CPU usage
- Increase update_interval_ms in settings
- Disable GPU monitoring if not needed

## Contributing

This is part of the SysSonic project. See the main README at the repository root for contribution guidelines.

## License

MIT or Apache-2.0 (same as the `tunes` library)

## Credits

Built with:
- [Tauri](https://tauri.app/) - Desktop application framework
- [React](https://react.dev/) - UI framework
- [tunes](https://github.com/sqrew/tunes) - Audio synthesis
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - System metrics

---

**"Listen to your system, not just monitor it!"** 🎧
