# kicad-ecs

Entity Component System (ECS) architecture for KiCad PCB design data.

## Overview

`kicad-ecs` provides an ECS-based approach to working with KiCad board data, enabling:
- Flexible component queries and filtering
- Extensible analysis systems
- High-performance data processing
- Clean separation of data and logic

## Why ECS for KiCad?

Traditional object-oriented approaches to PCB data can become rigid when dealing with the varied requirements of modern PCB analysis:
- Thermal simulation
- Signal integrity
- Mechanical stress analysis
- Manufacturing constraints
- EMI/EMC compliance

ECS allows you to compose these different aspects without complex inheritance hierarchies.

## Quick Start

```rust
use kicad_ecs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to running KiCad instance
    let mut client = KiCadClient::connect()?;
    
    // Get footprints from open board
    let footprints = client.get_footprints().await?;
    
    // Create ECS world and load components
    let mut pcb_world = PcbWorld::new();
    for fp in footprints {
        pcb_world.spawn_footprint(fp);
    }
    
    // Query components using ECS
    let mut query = pcb_world.world.query::<(&ComponentInfo, &Position)>();
    for (info, pos) in query.iter(&pcb_world.world) {
        println!("{} at ({:.1}, {:.1})mm", info.reference, pos.x, pos.y);
    }
    
    Ok(())
}
```

## Architecture

The library maps KiCad concepts to ECS:
- **KiCad PCB Footprint** → **ECS Entity**
- **Component Properties** → **ECS Components**
  - `ComponentId` - Unique identifier (UUID)
  - `ComponentInfo` - Reference, value, footprint name
  - `Position` - X, Y coordinates and rotation
  - `Layer` - PCB layer name
  - `ComponentDescription` - Component description
  - `ComponentFlags` - DNP, exclude from BOM, locked status
  - Type markers: `Resistor`, `Capacitor`, `IntegratedCircuit`, `Connector`

## Features

- ✅ Pure Rust implementation
- ✅ Async/await support
- ✅ Type-safe component queries
- ✅ Extensible component types
- ✅ Built on Bevy ECS
- ✅ Structured logging with `tracing`
- ✅ Comprehensive error handling

## Examples

See the `examples/` directory for:
- `real_kicad_ecs.rs` - Real KiCad integration that connects to a running KiCad instance and loads PCB data into ECS
- `tracing_demo.rs` - Structured logging demonstration with tracing

### Running Examples

```bash
# Real KiCad ECS integration (requires KiCad running with an open PCB)
cargo run --example real_kicad_ecs

# Tracing demo with debug logging
RUST_LOG=debug cargo run --example tracing_demo

# JSON structured output
KICAD_ECS_JSON_LOGS=1 cargo run --example tracing_demo
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Unlike some projects, we actually specify what we're looking for:**
- Additional analysis systems
- New component types for specialized domains
- Performance improvements
- Documentation improvements

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Acknowledgments

This project interfaces with [KiCad](https://kicad.org/), the open source EDA suite.