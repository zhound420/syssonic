#!/usr/bin/env python3
"""
SysSonic Demo Visualization
Shows what the system metrics → music mapping would look like
"""

def simulate_metrics():
    """Simulate different system states"""
    scenarios = {
        "Idle System": {
            "cpu_usage": 5.0,
            "memory_usage": 35.0,
            "disk_read_bytes": 1024,
            "disk_write_bytes": 512,
            "network_rx_bytes": 2048,
            "network_tx_bytes": 1024,
            "temperature": 40.0,
        },
        "Web Browsing": {
            "cpu_usage": 25.0,
            "memory_usage": 55.0,
            "disk_read_bytes": 102400,
            "disk_write_bytes": 51200,
            "network_rx_bytes": 5242880,  # 5 MB/s
            "network_tx_bytes": 524288,    # 512 KB/s
            "temperature": 48.0,
        },
        "Video Rendering": {
            "cpu_usage": 95.0,
            "memory_usage": 85.0,
            "disk_read_bytes": 10485760,   # 10 MB/s
            "disk_write_bytes": 20971520,  # 20 MB/s
            "network_rx_bytes": 1024,
            "network_tx_bytes": 512,
            "temperature": 72.0,
        },
        "Gaming Session": {
            "cpu_usage": 65.0,
            "memory_usage": 70.0,
            "disk_read_bytes": 5242880,
            "disk_write_bytes": 1048576,
            "network_rx_bytes": 2097152,   # 2 MB/s
            "network_tx_bytes": 524288,
            "temperature": 68.0,
        },
        "Compiling Large Project": {
            "cpu_usage": 100.0,
            "memory_usage": 90.0,
            "disk_read_bytes": 31457280,   # 30 MB/s
            "disk_write_bytes": 15728640,  # 15 MB/s
            "network_rx_bytes": 512,
            "network_tx_bytes": 256,
            "temperature": 78.0,
        },
    }
    return scenarios


def map_to_music(metrics):
    """Map system metrics to musical parameters"""
    
    # A minor pentatonic scale notes
    scale = ["A3", "C4", "D4", "E4", "G4", "A4", "C5", "D5", "E5", "G5", "A5", "C6", "D6"]
    
    # CPU → Melody
    scale_index = int((metrics["cpu_usage"] / 100.0) * (len(scale) - 1))
    melody_note = scale[scale_index]
    
    # Memory → Bass
    if metrics["memory_usage"] > 75:
        bass_note = "A2"
        bass_desc = "Deep, ominous"
    elif metrics["memory_usage"] > 50:
        bass_note = "E2"
        bass_desc = "Mid-range"
    else:
        bass_note = "E3"
        bass_desc = "Light, comfortable"
    bass_velocity = metrics["memory_usage"] / 100.0
    
    # Disk I/O → Rhythm
    total_io = (metrics["disk_read_bytes"] + metrics["disk_write_bytes"])
    io_normalized = min(total_io / 10_000_000, 1.0)
    
    if io_normalized < 0.3:
        rhythm = "Sparse (4-on-floor)"
    elif io_normalized < 0.6:
        rhythm = "Moderate (with fills)"
    else:
        rhythm = "Dense (complex polyrhythms)"
    
    # Network → Tempo
    total_network = metrics["network_rx_bytes"] + metrics["network_tx_bytes"]
    network_normalized = min(total_network / 5_000_000, 1.0)
    tempo = 90 + (network_normalized * 40)
    
    # Temperature → Effects
    temp_normalized = (metrics["temperature"] - 30) / 40
    temp_normalized = max(0, min(temp_normalized, 1.0))
    filter_cutoff = 400 + (temp_normalized * 2600)
    reverb_mix = temp_normalized * 50
    
    if temp_normalized < 0.3:
        atmosphere = "Tight, dry"
    elif temp_normalized < 0.7:
        atmosphere = "Moderate space"
    else:
        atmosphere = "Spacious, concerning"
    
    return {
        "melody_note": melody_note,
        "bass_note": bass_note,
        "bass_desc": bass_desc,
        "bass_velocity": bass_velocity,
        "rhythm": rhythm,
        "tempo": tempo,
        "filter_cutoff": filter_cutoff,
        "reverb_mix": reverb_mix,
        "atmosphere": atmosphere,
    }


def visualize_mapping(scenario_name, metrics):
    """Pretty-print the mapping"""
    print(f"\n{'='*60}")
    print(f"  {scenario_name}")
    print(f"{'='*60}")
    
    print("\n📊 SYSTEM METRICS:")
    print(f"  CPU Usage:      {metrics['cpu_usage']:>6.1f}%")
    print(f"  Memory Usage:   {metrics['memory_usage']:>6.1f}%")
    print(f"  Disk Read:      {metrics['disk_read_bytes']/1024:>6.0f} KB/s")
    print(f"  Disk Write:     {metrics['disk_write_bytes']/1024:>6.0f} KB/s")
    print(f"  Network RX:     {metrics['network_rx_bytes']/1024:>6.0f} KB/s")
    print(f"  Network TX:     {metrics['network_tx_bytes']/1024:>6.0f} KB/s")
    print(f"  Temperature:    {metrics['temperature']:>6.1f}°C")
    
    musical = map_to_music(metrics)
    
    print("\n🎵 MUSICAL TRANSLATION:")
    print(f"  Melody:         {musical['melody_note']} (from A minor pentatonic)")
    print(f"  Bass:           {musical['bass_note']} - {musical['bass_desc']}")
    print(f"                  Velocity: {musical['bass_velocity']:.0%}")
    print(f"  Rhythm:         {musical['rhythm']}")
    print(f"  Tempo:          {musical['tempo']:.0f} BPM")
    print(f"  Filter Cutoff:  {musical['filter_cutoff']:.0f} Hz")
    print(f"  Reverb:         {musical['reverb_mix']:.0f}%")
    print(f"  Atmosphere:     {musical['atmosphere']}")
    
    # Audio bar visualization
    print("\n🎧 AUDIO CHARACTERISTICS:")
    cpu_bar = '█' * int(metrics['cpu_usage'] / 5) + '░' * (20 - int(metrics['cpu_usage'] / 5))
    mem_bar = '█' * int(metrics['memory_usage'] / 5) + '░' * (20 - int(metrics['memory_usage'] / 5))
    io_bar = '█' * int((metrics['disk_read_bytes'] + metrics['disk_write_bytes']) / 1_500_000) 
    io_bar = io_bar[:20].ljust(20, '░')
    temp_bar = '█' * int((metrics['temperature'] - 30) / 2.5) + '░' * (20 - int((metrics['temperature'] - 30) / 2.5))
    
    print(f"  Pitch (CPU):    [{cpu_bar}]")
    print(f"  Bass (Memory):  [{mem_bar}]")
    print(f"  Rhythm (I/O):   [{io_bar}]")
    print(f"  Effects (Temp): [{temp_bar}]")


def main():
    print("=" * 60)
    print("  SysSonic - System Metrics Sonification Demo")
    print("  Showing how different workloads sound")
    print("=" * 60)
    
    scenarios = simulate_metrics()
    
    for scenario_name, metrics in scenarios.items():
        visualize_mapping(scenario_name, metrics)
    
    print("\n" + "="*60)
    print("  How to actually run SysSonic:")
    print("="*60)
    print("\n  Live mode:   cargo run --release -- live")
    print("  Export:      cargo run --release -- export -o output.wav")
    print("  Monitor:     cargo run --release -- monitor")
    print("  Test audio:  cargo run --release -- test")
    print("\n" + "="*60)


if __name__ == "__main__":
    main()
