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

fn main() -> Result<()> {
    // Connect to KiCad
    let client = KiCadClient::connect()?;
    
    // Load board into ECS
    let mut world = PcbWorld::from_board(client.get_board()?)?;
    
    // Query components
    let mut query = world.query::<(&Position, &Footprint)>();
    for (pos, footprint) in query.iter(&world) {
        println!("{} at ({}, {})", footprint.reference, pos.x, pos.y);
    }
    
    Ok(())
}
```

## Architecture

The library maps KiCad concepts to ECS:
- **KiCad PCB Component** → **ECS Entity**
- **Component Properties** → **ECS Components**
  - `Position` - X, Y coordinates and rotation
  - `Footprint` - Reference, value, library info
  - `Electrical` - Nets, pins, connections
  - `Thermal` - Heat dissipation, thermal pads
  - `Mechanical` - Height, weight, mounting

## Features

- ✅ Pure Rust implementation
- ✅ Async/await support
- ✅ Type-safe component queries
- ✅ Extensible component types
- ✅ Built on Bevy ECS

## Examples

See the `examples/` directory for:
- `basic.rs` - Simple connection and queries
- `analysis.rs` - Thermal and mechanical analysis
- `rules.rs` - Design rule checking with ECS

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