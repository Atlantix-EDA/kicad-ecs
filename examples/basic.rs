//! Basic example showing KiCad ECS integration
//!
//! This example demonstrates:
//! - Connecting to KiCad
//! - Loading board data into ECS
//! - Querying components with ECS systems

use kicad_ecs::prelude::*;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing with development-friendly settings
    kicad_ecs::tracing::init_for_examples();

    info!("Starting KiCad ECS Basic Example");
    println!("KiCad ECS Basic Example");
    println!("======================");
    
    // Try to connect to KiCad
    println!("Connecting to KiCad...");
    match kicad_ecs::client::KiCadClient::connect() {
        Ok(mut client) => {
            // Successfully connected - get version
            match client.get_version().await {
                Ok(version) => {
                    info!("Successfully connected to KiCad {}", version.full);
                    println!("Connected to KiCad {}", version.full);
                    
                    // Load real board data
                    println!("Loading board data...");
                    match client.get_board().await {
                        Ok(board) => {
                            println!("Board: {}", board.name);
                            
                            // Get footprints and load into ECS
                            match client.get_footprints().await {
                                Ok(footprints) => {
                                    println!("Found {} components on board", footprints.len());
                                    
                                    let mut world = PcbWorld::new();
                                    
                                    // Load real footprints into ECS
                                    for fp in footprints {
                                        world.add_component(
                                            fp.id,
                                            fp.reference,
                                            fp.value,
                                            fp.footprint_name,
                                            (fp.position.0, fp.position.1, fp.rotation),
                                            fp.layer,
                                            fp.description.unwrap_or_default(),
                                            (fp.exclude_from_bom, fp.do_not_populate, fp.locked),
                                        );
                                    }
                                    
                                    analyze_components(&mut world);
                                },
                                Err(e) => {
                                    warn!("Failed to get footprints: {}", e);
                                    println!("Failed to get footprints: {}", e);
                                    fallback_demo();
                                }
                            }
                        },
                        Err(e) => {
                            warn!("Failed to get board: {}", e);
                            println!("Failed to get board: {}. Make sure a PCB is open!", e);
                            fallback_demo();
                        }
                    }
                },
                Err(e) => {
                    warn!("Failed to get KiCad version: {}", e);
                    println!("Failed to get version: {}", e);
                    fallback_demo();
                }
            }
        },
        Err(e) => {
            warn!("Failed to connect to KiCad: {}", e);
            println!("Failed to connect to KiCad: {}", e);
            println!("Make sure KiCad is running with API enabled!");
            fallback_demo();
        }
    }
    
    Ok(())
}

/// Fallback to demo data if KiCad is not available
fn fallback_demo() {
    println!("\n=== Demo Mode ===");
    println!("Running with demo data since KiCad is not available\n");
    
    let mut world = PcbWorld::new();
    create_demo_data(&mut world);
    analyze_components(&mut world);
}

/// Analyze components in the ECS world
fn analyze_components(world: &mut PcbWorld) {
    println!("Analyzing components...");
    
    // Query all components
    let mut query = world.world.query::<(&ComponentInfo, &Position)>();
    let component_count = query.iter(&world.world).count();
    println!("Found {} components", component_count);
    
    // List components
    for (info, pos) in query.iter(&world.world) {
        println!("  {} ({}) at ({:.1}, {:.1})", 
                 info.reference, info.value, pos.x, pos.y);
    }
}

/// Create some demo data for testing without KiCad connection
fn create_demo_data(world: &mut PcbWorld) {
    use kicad_ecs::components::*;
    
    // Add a resistor
    world.world.spawn((
        ComponentId { uuid: "r1-uuid".to_string() },
        ComponentInfo {
            reference: "R1".to_string(),
            value: "10k".to_string(),
            footprint_name: "Resistor_SMD:R_0603".to_string(),
        },
        Position { x: 10.0, y: 20.0, rotation: 0.0 },
        Layer { layer_name: "F.Cu".to_string() },
        ComponentFlags {
            exclude_from_bom: false,
            do_not_populate: false,
            locked: false,
        },
    ));
    
    // Add a capacitor
    world.world.spawn((
        ComponentId { uuid: "c1-uuid".to_string() },
        ComponentInfo {
            reference: "C1".to_string(),
            value: "100nF".to_string(),
            footprint_name: "Capacitor_SMD:C_0603".to_string(),
        },
        Position { x: 15.0, y: 25.0, rotation: 90.0 },
        Layer { layer_name: "F.Cu".to_string() },
        ComponentFlags {
            exclude_from_bom: false,
            do_not_populate: false,
            locked: false,
        },
    ));
    
    // Add a mounting hole
    world.world.spawn((
        ComponentId { uuid: "h1-uuid".to_string() },
        ComponentInfo {
            reference: "H1".to_string(),
            value: "MountingHole".to_string(),
            footprint_name: "MountingHole:MountingHole_3.2mm_M3".to_string(),
        },
        Position { x: 5.0, y: 5.0, rotation: 0.0 },
        Layer { layer_name: "F.Cu".to_string() },
        MountingHole {
            diameter_mm: 3.2,
            screw_size: "M3".to_string(),
        },
    ));
}