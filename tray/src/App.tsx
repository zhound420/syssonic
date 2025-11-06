import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

interface SystemMetrics {
  cpu_usage: number;
  memory_usage: number;
  memory_total: number;
  disk_read: number;
  disk_write: number;
  network_rx: number;
  network_tx: number;
  temperature: number;
  load_avg_1: number;
  load_avg_5: number;
  load_avg_15: number;
  swap_total: number;
  swap_used: number;
  per_core_usage: number[];
  process_count: number;
  gpu_nvidia?: {
    name: string;
    utilization: number;
    temperature: number;
    memory_used: number;
    memory_total: number;
    power_draw: number;
    fan_speed?: number;
  };
  gpu_amd?: {
    utilization: number;
    temperature: number;
    power_draw: number;
    memory_used: number;
    memory_total: number;
  };
  battery?: {
    state_of_charge: number;
    state: string;
    power_rate: number;
    temperature?: number;
    time_to_full?: number;
    time_to_empty?: number;
  };
  fan_speeds?: Array<{
    name: string;
    rpm: number;
  }>;
  top_processes: Array<{
    pid: number;
    name: string;
    cpu_usage: number;
    memory_mb: number;
  }>;
}

interface MusicalParams {
  tempo: number;
  melody_notes: number[];
  bass_note: number;
  percussion_density: number;
  filter_cutoff: number;
  reverb_amount: number;
}

interface AudioState {
  playing: boolean;
  volume: number;
}

interface AppConfig {
  volume: number;
  auto_start: boolean;
  theme: string;
  update_interval_ms: number;
  enable_gpu_monitoring: boolean;
}

function App() {
  const [metrics, setMetrics] = useState<SystemMetrics | null>(null);
  const [musicalParams, setMusicalParams] = useState<MusicalParams | null>(null);
  const [audioState, setAudioState] = useState<AudioState>({ playing: false, volume: 0.8 });
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [activeTab, setActiveTab] = useState<"dashboard" | "settings">("dashboard");

  // Load config on mount
  useEffect(() => {
    const loadConfig = async () => {
      try {
        const cfg = await invoke<AppConfig>("get_config");
        setConfig(cfg);
      } catch (e) {
        console.error("Failed to load config:", e);
      }
    };
    loadConfig();
  }, []);

  // Poll metrics every 2 seconds
  useEffect(() => {
    const pollMetrics = async () => {
      try {
        const m = await invoke<SystemMetrics>("get_current_metrics");
        setMetrics(m);

        const mp = await invoke<MusicalParams>("get_musical_params");
        setMusicalParams(mp);
      } catch (e) {
        console.error("Failed to poll metrics:", e);
      }
    };

    pollMetrics();
    const interval = setInterval(pollMetrics, 2000);
    return () => clearInterval(interval);
  }, []);

  // Poll audio state
  useEffect(() => {
    const pollAudioState = async () => {
      try {
        const state = await invoke<AudioState>("get_audio_state");
        setAudioState(state);
      } catch (e) {
        console.error("Failed to poll audio state:", e);
      }
    };

    pollAudioState();
    const interval = setInterval(pollAudioState, 1000);
    return () => clearInterval(interval);
  }, []);

  // Listen to tray commands
  useEffect(() => {
    const setupListeners = async () => {
      const unlisten = await listen("tray-command", (event: any) => {
        const payload = event.payload;

        if (payload === "start") {
          handleStart();
        } else if (payload === "stop") {
          handleStop();
        } else if (payload === "export") {
          handleExport();
        } else if (payload === "settings") {
          setActiveTab("settings");
        } else if (typeof payload === "object" && payload.action === "volume") {
          handleVolumeChange(payload.value);
        }
      });

      return () => {
        unlisten();
      };
    };

    setupListeners();
  }, []);

  const handleStart = async () => {
    try {
      await invoke("start_audio");
    } catch (e) {
      console.error("Failed to start audio:", e);
    }
  };

  const handleStop = async () => {
    try {
      await invoke("stop_audio");
    } catch (e) {
      console.error("Failed to stop audio:", e);
    }
  };

  const handlePause = async () => {
    try {
      await invoke("pause_audio");
    } catch (e) {
      console.error("Failed to pause audio:", e);
    }
  };

  const handleResume = async () => {
    try {
      await invoke("resume_audio");
    } catch (e) {
      console.error("Failed to resume audio:", e);
    }
  };

  const handleVolumeChange = async (vol: number) => {
    try {
      await invoke("set_volume", { volume: vol });
      setAudioState({ ...audioState, volume: vol });
    } catch (e) {
      console.error("Failed to set volume:", e);
    }
  };

  const handleExport = async () => {
    try {
      const timestamp = new Date().toISOString().replace(/[:.]/g, "-");
      const filename = `syssonic-${timestamp}.wav`;
      await invoke("export_audio", {
        path: filename,
        format: "wav",
        bars: 4
      });
      alert(`Exported to ${filename}`);
    } catch (e) {
      console.error("Failed to export:", e);
      alert(`Export failed: ${e}`);
    }
  };

  const formatBytes = (bytes: number) => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
    return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
  };

  return (
    <div className="app">
      <header>
        <h1>🎵 SysSonic</h1>
        <nav>
          <button
            className={activeTab === "dashboard" ? "active" : ""}
            onClick={() => setActiveTab("dashboard")}
          >
            Dashboard
          </button>
          <button
            className={activeTab === "settings" ? "active" : ""}
            onClick={() => setActiveTab("settings")}
          >
            Settings
          </button>
        </nav>
      </header>

      {activeTab === "dashboard" ? (
        <main className="dashboard">
          {/* Audio Controls */}
          <section className="controls-section">
            <h2>Audio Controls</h2>
            <div className="controls">
              <button onClick={handleStart} disabled={audioState.playing}>
                ▶ Start
              </button>
              <button onClick={handleStop} disabled={!audioState.playing}>
                ⏹ Stop
              </button>
              <button onClick={handlePause} disabled={!audioState.playing}>
                ⏸ Pause
              </button>
              <button onClick={handleResume} disabled={audioState.playing}>
                ▶ Resume
              </button>
              <button onClick={handleExport}>
                💾 Export
              </button>
            </div>
            <div className="volume-control">
              <label>Volume: {Math.round(audioState.volume * 100)}%</label>
              <input
                type="range"
                min="0"
                max="1"
                step="0.01"
                value={audioState.volume}
                onChange={(e) => handleVolumeChange(parseFloat(e.target.value))}
              />
            </div>
            <div className={`status-badge ${audioState.playing ? "playing" : "stopped"}`}>
              {audioState.playing ? "🔊 Playing" : "🔇 Stopped"}
            </div>
          </section>

          {/* System Metrics */}
          {metrics && (
            <>
              <section className="metrics-grid">
                <div className="metric-card">
                  <h3>CPU Usage</h3>
                  <div className="metric-value">{metrics.cpu_usage.toFixed(1)}%</div>
                  <div className="metric-bar">
                    <div className="bar-fill" style={{ width: `${metrics.cpu_usage}%` }}></div>
                  </div>
                </div>

                <div className="metric-card">
                  <h3>Memory</h3>
                  <div className="metric-value">{metrics.memory_usage.toFixed(1)}%</div>
                  <div className="metric-bar">
                    <div className="bar-fill" style={{ width: `${metrics.memory_usage}%` }}></div>
                  </div>
                  <div className="metric-detail">
                    {formatBytes(metrics.memory_total * metrics.memory_usage / 100)} / {formatBytes(metrics.memory_total)}
                  </div>
                </div>

                <div className="metric-card">
                  <h3>Temperature</h3>
                  <div className="metric-value">{metrics.temperature.toFixed(1)}°C</div>
                  <div className="metric-bar">
                    <div className="bar-fill" style={{ width: `${Math.min(metrics.temperature / 100 * 100, 100)}%` }}></div>
                  </div>
                </div>

                <div className="metric-card">
                  <h3>Load Average</h3>
                  <div className="metric-value">{metrics.load_avg_1.toFixed(2)}</div>
                  <div className="metric-detail">
                    1m: {metrics.load_avg_1.toFixed(2)} | 5m: {metrics.load_avg_5.toFixed(2)} | 15m: {metrics.load_avg_15.toFixed(2)}
                  </div>
                </div>

                <div className="metric-card">
                  <h3>Disk I/O</h3>
                  <div className="metric-detail">
                    ↓ Read: {formatBytes(metrics.disk_read)}/s
                  </div>
                  <div className="metric-detail">
                    ↑ Write: {formatBytes(metrics.disk_write)}/s
                  </div>
                </div>

                <div className="metric-card">
                  <h3>Network</h3>
                  <div className="metric-detail">
                    ↓ RX: {formatBytes(metrics.network_rx)}/s
                  </div>
                  <div className="metric-detail">
                    ↑ TX: {formatBytes(metrics.network_tx)}/s
                  </div>
                </div>
              </section>

              {/* Per-Core CPU */}
              {metrics.per_core_usage.length > 0 && (
                <section className="per-core-section">
                  <h2>Per-Core CPU Usage</h2>
                  <div className="core-grid">
                    {metrics.per_core_usage.map((usage, idx) => (
                      <div key={idx} className="core-card">
                        <div className="core-label">Core {idx}</div>
                        <div className="core-value">{usage.toFixed(0)}%</div>
                        <div className="core-bar">
                          <div className="bar-fill" style={{ width: `${usage}%` }}></div>
                        </div>
                      </div>
                    ))}
                  </div>
                </section>
              )}

              {/* GPU Info */}
              {(metrics.gpu_nvidia || metrics.gpu_amd) && (
                <section className="gpu-section">
                  <h2>GPU Metrics</h2>
                  {metrics.gpu_nvidia && (
                    <div className="gpu-card">
                      <h3>NVIDIA {metrics.gpu_nvidia.name}</h3>
                      <div className="gpu-stats">
                        <div>Utilization: {metrics.gpu_nvidia.utilization.toFixed(0)}%</div>
                        <div>Temperature: {metrics.gpu_nvidia.temperature.toFixed(1)}°C</div>
                        <div>Power: {metrics.gpu_nvidia.power_draw.toFixed(1)}W</div>
                        <div>
                          VRAM: {formatBytes(metrics.gpu_nvidia.memory_used)} / {formatBytes(metrics.gpu_nvidia.memory_total)}
                        </div>
                        {metrics.gpu_nvidia.fan_speed && (
                          <div>Fan: {metrics.gpu_nvidia.fan_speed.toFixed(0)}%</div>
                        )}
                      </div>
                    </div>
                  )}
                  {metrics.gpu_amd && (
                    <div className="gpu-card">
                      <h3>AMD GPU</h3>
                      <div className="gpu-stats">
                        <div>Utilization: {metrics.gpu_amd.utilization.toFixed(0)}%</div>
                        <div>Temperature: {metrics.gpu_amd.temperature.toFixed(1)}°C</div>
                        <div>Power: {metrics.gpu_amd.power_draw.toFixed(1)}W</div>
                        <div>
                          VRAM: {formatBytes(metrics.gpu_amd.memory_used)} / {formatBytes(metrics.gpu_amd.memory_total)}
                        </div>
                      </div>
                    </div>
                  )}
                </section>
              )}

              {/* Battery Info */}
              {metrics.battery && (
                <section className="battery-section">
                  <h2>Battery</h2>
                  <div className="battery-card">
                    <div className="battery-charge">{metrics.battery.state_of_charge.toFixed(0)}%</div>
                    <div className="battery-state">{metrics.battery.state}</div>
                    <div className="battery-bar">
                      <div className="bar-fill" style={{ width: `${metrics.battery.state_of_charge}%` }}></div>
                    </div>
                    <div className="battery-details">
                      <div>Power: {metrics.battery.power_rate.toFixed(1)}W</div>
                      {metrics.battery.temperature && (
                        <div>Temp: {metrics.battery.temperature.toFixed(1)}°C</div>
                      )}
                      {metrics.battery.time_to_full && (
                        <div>Time to Full: {Math.round(metrics.battery.time_to_full)}m</div>
                      )}
                      {metrics.battery.time_to_empty && (
                        <div>Time to Empty: {Math.round(metrics.battery.time_to_empty)}m</div>
                      )}
                    </div>
                  </div>
                </section>
              )}

              {/* Top Processes */}
              {metrics.top_processes.length > 0 && (
                <section className="processes-section">
                  <h2>Top Processes</h2>
                  <table className="processes-table">
                    <thead>
                      <tr>
                        <th>PID</th>
                        <th>Name</th>
                        <th>CPU</th>
                        <th>Memory</th>
                      </tr>
                    </thead>
                    <tbody>
                      {metrics.top_processes.map((proc) => (
                        <tr key={proc.pid}>
                          <td>{proc.pid}</td>
                          <td>{proc.name}</td>
                          <td>{proc.cpu_usage.toFixed(1)}%</td>
                          <td>{proc.memory_mb.toFixed(0)} MB</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </section>
              )}

              {/* Musical Parameters */}
              {musicalParams && (
                <section className="musical-section">
                  <h2>Musical Parameters</h2>
                  <div className="musical-grid">
                    <div className="musical-card">
                      <div className="musical-label">Tempo</div>
                      <div className="musical-value">{musicalParams.tempo.toFixed(0)} BPM</div>
                    </div>
                    <div className="musical-card">
                      <div className="musical-label">Melody Notes</div>
                      <div className="musical-value">{musicalParams.melody_notes.length} notes</div>
                    </div>
                    <div className="musical-card">
                      <div className="musical-label">Bass Freq</div>
                      <div className="musical-value">{musicalParams.bass_note.toFixed(1)} Hz</div>
                    </div>
                    <div className="musical-card">
                      <div className="musical-label">Percussion</div>
                      <div className="musical-value">{(musicalParams.percussion_density * 100).toFixed(0)}%</div>
                    </div>
                    <div className="musical-card">
                      <div className="musical-label">Filter</div>
                      <div className="musical-value">{musicalParams.filter_cutoff.toFixed(0)} Hz</div>
                    </div>
                    <div className="musical-card">
                      <div className="musical-label">Reverb</div>
                      <div className="musical-value">{(musicalParams.reverb_amount * 100).toFixed(0)}%</div>
                    </div>
                  </div>
                </section>
              )}
            </>
          )}
        </main>
      ) : (
        <main className="settings">
          <h2>Settings</h2>
          {config && (
            <div className="settings-form">
              <div className="setting-item">
                <label>Theme</label>
                <select
                  value={config.theme}
                  onChange={async (e) => {
                    await invoke("update_config_field", { field: "theme", value: e.target.value });
                    setConfig({ ...config, theme: e.target.value });
                  }}
                >
                  <option value="dark">Dark</option>
                  <option value="light">Light</option>
                </select>
              </div>

              <div className="setting-item">
                <label>
                  <input
                    type="checkbox"
                    checked={config.auto_start}
                    onChange={async (e) => {
                      await invoke("update_config_field", { field: "auto_start", value: e.target.checked });
                      setConfig({ ...config, auto_start: e.target.checked });
                    }}
                  />
                  Auto-start on system boot
                </label>
              </div>

              <div className="setting-item">
                <label>
                  <input
                    type="checkbox"
                    checked={config.enable_gpu_monitoring}
                    onChange={async (e) => {
                      await invoke("update_config_field", { field: "enable_gpu_monitoring", value: e.target.checked });
                      setConfig({ ...config, enable_gpu_monitoring: e.target.checked });
                    }}
                  />
                  Enable GPU monitoring
                </label>
              </div>

              <div className="setting-item">
                <label>Update interval (ms)</label>
                <input
                  type="number"
                  value={config.update_interval_ms}
                  onChange={async (e) => {
                    const val = parseInt(e.target.value);
                    await invoke("update_config_field", { field: "update_interval_ms", value: val });
                    setConfig({ ...config, update_interval_ms: val });
                  }}
                />
              </div>
            </div>
          )}
        </main>
      )}
    </div>
  );
}

export default App;
