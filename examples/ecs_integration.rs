//! # KiCad ECS Integration Example
//!
//! This example demonstrates how to read component data from a real KiCad board
//! and organize it using an Entity Component System (ECS) architecture. The focus is on
//! using an ECS to manage PCB components, their positions, and associated metadata.
//!
//! What this example does:
//! - Connects to a running KiCad instance
//! - Retrieve board data and footprints
//! - Organize components into entities with various components
//! - Analyze and display component data in a structured format
//!
//! The ECS in this example is Bevy ECS, but other ECS libraries could be used as well such
//! as SPECS or HECS. The goal is to show how to structure PCB data in an ECS format.
//!
//! To run this example, make sure you have:
//! 1. KiCad installed and running
//! 2. A PCB board open in KiCad
//!
//! Then execute:
//! ```bash
//! cargo run --example ecs_integration
//! ```

use kicad_ecs::prelude::*;
// use kicad_ecs::components::*; // imported via prelude
use prettytable::{Table, row, format, Cell};
use tracing::{debug, error, info, instrument, warn};
use std::time::Duration;

/// Check if a footprint is a mounting hole based on naming patterns
fn is_mounting_hole(footprint_data: &kicad_ecs::client::FootprintData) -> bool {
    let ref_name = &footprint_data.reference;
    let value = &footprint_data.value;
    let footprint = &footprint_data.footprint_name;
    
    ref_name.starts_with("H") || 
    ref_name.starts_with("MH") ||
    ref_name.contains("MountingHole") ||
    value.contains("MountingHole") ||
    footprint.contains("MountingHole") ||
    footprint.contains("Hole_")
}

/// Enhanced PCB World with analysis capabilities
pub struct EnhancedPcbWorld {
    pub world: World,
    component_count: usize,
    mounting_hole_count: usize,
}

impl EnhancedPcbWorld {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            component_count: 0,
            mounting_hole_count: 0,
        }
    }
    
    #[instrument(skip(self))]
    pub fn set_mounting_hole_count(&mut self, count: usize) {
        debug!("Setting mounting hole count to {}", count);
        self.mounting_hole_count = count;
    }
    
    #[instrument(skip(self), fields(reference = %reference, footprint = %footprint))]
    pub fn add_mounting_hole(&mut self, id: String, reference: String, footprint: String, 
                            x: f64, y: f64, rotation: f64, layer: String) {
        debug!("Adding mounting hole: {} at ({:.1}, {:.1})", reference, x, y);
        
        let (diameter_mm, screw_size) = Self::parse_mounting_hole_size(&footprint);
        
        let position = Position { x, y, rotation };
        let layer_comp = Layer { layer_name: layer };
        let comp_id = ComponentId { uuid: id };
        let mounting_hole = MountingHole {
            diameter_mm,
            screw_size: screw_size.clone(),
        };
        
        self.world.spawn((comp_id, position, layer_comp, mounting_hole));
        debug!("Added mounting hole with diameter {:.1}mm, screw size {}", diameter_mm, screw_size);
    }
    
    /// Parse mounting hole size from footprint name
    fn parse_mounting_hole_size(footprint: &str) -> (f64, String) {
        // Parse common mounting hole footprint patterns
        // Examples: MountingHole_3.2mm, MountingHole_2.2mm_M2, Hole_2.7mm, etc.
        
        if let Some(mm_pos) = footprint.find("mm") {
            let before_mm = &footprint[..mm_pos];
            let mut start = 0;
            for (i, c) in before_mm.char_indices().rev() {
                if c.is_ascii_digit() || c == '.' {
                    start = i;
                } else {
                    break;
                }
            }
            
            if let Ok(diameter) = before_mm[start..].parse::<f64>() {
                let screw_size = match diameter {
                    d if d >= 2.0 && d <= 2.4 => "M2".to_string(),
                    d if d >= 2.5 && d <= 3.4 => "M3".to_string(), 
                    d if d >= 3.5 && d <= 4.4 => "M4".to_string(),
                    d if d >= 4.5 && d <= 5.4 => "M5".to_string(),
                    d if d >= 5.5 && d <= 6.4 => "M6".to_string(),
                    _ => format!("{}mm", diameter),
                };
                return (diameter, screw_size);
            }
        }
        
        (0.0, "Unknown".to_string())
    }
    
    /// Factory method to create and add components to the ECS world
    #[instrument(skip(self), fields(reference = %reference, value = %value))]
    pub fn add_component(&mut self, id: String, reference: String, value: String, 
                        footprint: String, x: f64, y: f64, rotation: f64, layer: String,
                        description: String, exclude_from_bom: bool, dnp: bool) {
        
        debug!("Adding component: {} = {} at ({:.1}, {:.1})", reference, value, x, y);
        
        let position = Position { x, y, rotation };
        let info = ComponentInfo {
            reference,
            value,
            footprint_name: footprint,
        };
        let desc = ComponentDescription { description };
        let layer_comp = Layer { layer_name: layer };
        let flags = ComponentFlags {
            exclude_from_bom,
            do_not_populate: dnp,
            locked: false, // Default to unlocked
        };
        let comp_id = ComponentId { uuid: id };
        
        self.world.spawn((comp_id, position, info, desc, layer_comp, flags));
        self.component_count += 1;
    }
    
    /// Load footprint data from KiCad client
    #[instrument(skip(self, footprints))]
    pub fn load_footprints(&mut self, footprints: Vec<kicad_ecs::client::FootprintData>) -> Result<()> {
        info!("Loading {} footprints into ECS world", footprints.len());
        
        let mut mounting_hole_count = 0;
        
        for fp in footprints.iter() {
            if is_mounting_hole(fp) {
                mounting_hole_count += 1;
                
                self.add_mounting_hole(
                    fp.id.clone(),
                    fp.reference.clone(),
                    fp.footprint_name.clone(),
                    fp.position.0,
                    fp.position.1,
                    fp.rotation,
                    fp.layer.clone(),
                );
                continue; // Skip adding as regular component
            }
            
            self.add_component(
                fp.id.clone(),
                fp.reference.clone(),
                fp.value.clone(),
                fp.footprint_name.clone(),
                fp.position.0,
                fp.position.1,
                fp.rotation,
                fp.layer.clone(),
                fp.description.clone().unwrap_or_default(),
                fp.exclude_from_bom,
                fp.do_not_populate,
            );
        }
        
        self.set_mounting_hole_count(mounting_hole_count);
        if mounting_hole_count > 0 {
            info!("Found {} mounting holes (excluded from component count)", mounting_hole_count);
        }
        
        info!("Successfully loaded {} components and {} mounting holes", 
              self.component_count, mounting_hole_count);
        Ok(())
    }
    
    #[instrument(skip(self))]
    pub fn print_summary(&mut self) {
        info!("Generating component summary");
        
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        
        table.add_row(row![b->"PCB Component Summary"]);
        table.add_row(row!["Total Components", self.component_count]);
        
        // Count by layer using Bevy ECS query
        let mut front_count = 0;
        let mut back_count = 0;
        
        let mut query = self.world.query::<&Layer>();
        for layer in query.iter(&self.world) {
            match layer.layer_name.as_str() {
                "F.Cu" => front_count += 1,
                "B.Cu" => back_count += 1,
                _ => {}
            }
        }
        
        table.add_row(row!["Front Layer (F.Cu)", front_count]);
        table.add_row(row!["Back Layer (B.Cu)", back_count]);
        
        // Count special flags
        let mut dnp_count = 0;
        let mut exclude_bom_count = 0;
        
        let mut flag_query = self.world.query::<&ComponentFlags>();
        for flags in flag_query.iter(&self.world) {
            if flags.do_not_populate { dnp_count += 1; }
            if flags.exclude_from_bom { exclude_bom_count += 1; }
        }
        
        if dnp_count > 0 {
            table.add_row(row!["Do Not Populate (DNP)", dnp_count]);
        }
        if exclude_bom_count > 0 {
            table.add_row(row!["Exclude from BOM", exclude_bom_count]);
        }
        
        if self.mounting_hole_count > 0 {
            table.add_row(row!["Mounting Holes", self.mounting_hole_count]);
        }
        
        println!("\nBoard Analysis:");
        table.printstd();
    }
    
    #[instrument(skip(self))]
    pub fn print_components(&mut self) {
        info!("Generating component list");
        
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        
        table.add_row(row![b->"Ref", b->"Value", b->"Footprint", b->"Description", b->"X(mm)", b->"Y(mm)", b->"Rot°", b->"Layer"]);
        
        let mut query = self.world.query::<(&ComponentInfo, &Position, &Layer, &ComponentDescription)>();
        for (info, pos, layer, desc) in query.iter(&self.world) {
            
            let layer_name = &layer.layer_name;
            
            // Create a formatted description that wraps at word boundaries
            let desc_wrapped = if desc.description.len() > 12 {
                let words: Vec<&str> = desc.description.split_whitespace().collect();
                let mut lines = Vec::new();
                let mut current_line = String::new();
                
                for word in words {
                    if current_line.len() + word.len() + 1 > 12 {
                        if !current_line.is_empty() {
                            lines.push(current_line);
                            current_line = String::new();
                        }
                    }
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                }
                if !current_line.is_empty() {
                    lines.push(current_line);
                }
                lines.join("\n")
            } else {
                desc.description.clone()
            };
            
            let desc_cell = if desc_wrapped.contains('\n') {
                let lines: Vec<&str> = desc_wrapped.split('\n').collect();
                let padded_lines: Vec<String> = lines.iter()
                    .map(|line| format!("{:<14}", line))
                    .collect();
                Cell::new(&padded_lines.join("\n"))
            } else {
                Cell::new(&format!("{:<14}", desc_wrapped))
            };
            
            // Truncate columns for reasonable width
            let value_short = if info.value.len() > 12 {
                format!("{}...", &info.value[..9])
            } else {
                info.value.clone()
            };
            
            let footprint_short = if info.footprint_name.len() > 20 {
                format!("{}...", &info.footprint_name[..17])
            } else {
                info.footprint_name.clone()
            };
            
            table.add_row(row![
                info.reference,
                value_short,
                footprint_short,
                desc_cell,
                format!("{:.1}", pos.x),
                format!("{:.1}", pos.y),
                format!("{:.0}", pos.rotation),
                layer_name
            ]);
        }
        
        println!("\nComponent List (all {} components):", self.component_count);
        table.printstd();
    }
    
    #[instrument(skip(self))]
    pub fn print_mounting_holes(&mut self) {
        if self.mounting_hole_count == 0 {
            debug!("No mounting holes to display");
            return;
        }
        
        info!("Generating mounting holes analysis");
        
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        
        table.add_row(row![b->"Ref", b->"Screw Size", b->"Diameter (mm)", b->"X(mm)", b->"Y(mm)", b->"Layer"]);
        
        let mut query = self.world.query::<(&ComponentId, &MountingHole, &Position, &Layer)>();
        for (comp_id, hole, pos, layer) in query.iter(&self.world) {
            // Extract reference from component ID or use a default
            let reference = comp_id.uuid.clone(); // TODO: Could store reference separately
            
            table.add_row(row![
                reference,
                hole.screw_size,
                format!("{:.1}", hole.diameter_mm),
                format!("{:.1}", pos.x),
                format!("{:.1}", pos.y),
                layer.layer_name
            ]);
        }
        
        println!("\nMounting Holes Analysis:");
        table.printstd();
    }
}

/// Client state machine states
#[derive(Debug, Clone, PartialEq)]
enum ClientState {
    Disconnected,
    Connecting,
    Connected,
    LoadingComponents,
    AnalyzingBoard,
    Error(String),
    Terminated,
}

/// Supervisor to manage the KiCad connection and ECS world
struct Supervisor {
    state: ClientState,
    client: Option<kicad_ecs::client::KiCadClient>,
    pcb_world: EnhancedPcbWorld,
}

impl Supervisor {
    fn new() -> Self {
        Self {
            state: ClientState::Disconnected,
            client: None,
            pcb_world: EnhancedPcbWorld::new(),
        }
    }

    #[instrument(skip(self))]
    async fn try_connect(&mut self) -> Result<(), String> {
        info!("Attempting to connect to KiCad");
        
        match kicad_ecs::client::KiCadClient::connect() {
            Ok(client) => {
                info!("Successfully connected to KiCad");
                self.client = Some(client);
                self.state = ClientState::Connected;
                Ok(())
            }
            Err(e) => {
                error!("Failed to connect to KiCad: {}", e);
                self.state = ClientState::Error(format!("{:?}", e));
                Err(format!("Failed to connect to KiCad: {:?}", e))
            }
        }
    }

    #[instrument(skip(self))]
    async fn load_board_components(&mut self) -> Result<(), String> {
        info!("Loading board components");
        
        if let Some(_client) = &mut self.client {
            // TODO: Once the client is fully implemented, this will work:
            /*
            match client.get_board().await {
                Ok(board) => {
                    info!("Board: {}", board.name);
                    info!("Project: {}", board.project_name.unwrap_or_else(|| "Unknown".to_string()));
                    
                    match client.get_footprints().await {
                        Ok(footprints) => {
                            info!("Found {} footprints, loading into ECS...", footprints.len());
                            self.pcb_world.load_footprints(footprints)?;
                            Ok(())
                        }
                        Err(e) => Err(format!("Failed to get footprints: {:?}", e))
                    }
                }
                Err(e) => Err(format!("Failed to get open board: {:?}", e))
            }
            */
            
            // For now, create demo data since the client isn't fully implemented
            warn!("KiCad client not fully implemented yet - using demo data");
            self.create_demo_data();
            Ok(())
        } else {
            Err("KiCad client is not initialized.".to_string())
        }
    }
    
    /// Create demo data for testing without full KiCad connection
    #[instrument(skip(self))]
    fn create_demo_data(&mut self) {
        info!("Creating demo component data");
        
        // Add various electronic components
        self.pcb_world.add_component(
            "r1-uuid".to_string(),
            "R1".to_string(),
            "10k".to_string(),
            "Resistor_SMD:R_0603_1608Metric".to_string(),
            10.0, 20.0, 0.0,
            "F.Cu".to_string(),
            "Pull-up resistor".to_string(),
            false, false,
        );
        
        self.pcb_world.add_component(
            "c1-uuid".to_string(),
            "C1".to_string(),
            "100nF".to_string(),
            "Capacitor_SMD:C_0603_1608Metric".to_string(),
            15.0, 25.0, 90.0,
            "F.Cu".to_string(),
            "Decoupling capacitor".to_string(),
            false, false,
        );
        
        self.pcb_world.add_component(
            "u1-uuid".to_string(),
            "U1".to_string(),
            "STM32F401RCT6".to_string(),
            "Package_QFP:LQFP-64_10x10mm_P0.5mm".to_string(),
            30.0, 30.0, 0.0,
            "F.Cu".to_string(),
            "32-bit ARM Cortex-M4 microcontroller".to_string(),
            false, false,
        );
        
        // Add some back-side components
        self.pcb_world.add_component(
            "r2-uuid".to_string(),
            "R2".to_string(),
            "4.7k".to_string(),
            "Resistor_SMD:R_0603_1608Metric".to_string(),
            5.0, 15.0, 180.0,
            "B.Cu".to_string(),
            "Current limiting resistor".to_string(),
            false, false,
        );
        
        // Add a mounting hole
        self.pcb_world.add_mounting_hole(
            "h1-uuid".to_string(),
            "H1".to_string(),
            "MountingHole:MountingHole_3.2mm_M3_Pad".to_string(),
            5.0, 5.0, 0.0,
            "F.Cu".to_string(),
        );
        
        self.pcb_world.set_mounting_hole_count(1);
        
        info!("Created demo data with {} components and {} mounting holes", 
              self.pcb_world.component_count, self.pcb_world.mounting_hole_count);
    }

    #[instrument(skip(self))]
    async fn update(&mut self) {
        match &mut self.state {
            ClientState::Disconnected => {
                info!("Client is disconnected");
                println!("Client is disconnected.");
                self.state = ClientState::Connecting;
            }
            ClientState::Connecting => {
                info!("Connecting to KiCad");
                println!("Connecting to KiCad...");
                
                if let Err(err) = self.try_connect().await {
                    error!("Connection failed: {}", err);
                    println!("Error: {}", err);
                    println!("   Make sure KiCad is running!");
                    self.state = ClientState::Error(err);
                } else {
                    info!("Connected successfully");
                    // TODO: Get version when client is implemented
                    // println!("Connected to KiCad version: {}", version);
                    println!("Connected to KiCad (demo mode)");
                    self.state = ClientState::LoadingComponents;
                }
            }
            ClientState::Connected => {
                debug!("Moving from Connected to LoadingComponents");
                self.state = ClientState::LoadingComponents;
            }
            ClientState::LoadingComponents => {
                info!("Loading board components");
                println!("\nLoading board components...");
                
                match self.load_board_components().await {
                    Ok(_) => {
                        info!("Components loaded successfully");
                        self.state = ClientState::AnalyzingBoard;
                    }
                    Err(e) => {
                        error!("Failed to load components: {}", e);
                        println!("Error: {}", e);
                        println!("   Make sure you have a PCB board open in KiCad!");
                        self.state = ClientState::Error(e);
                    }
                }
            }
            ClientState::AnalyzingBoard => {
                info!("Analyzing board with ECS queries");
                println!("\nAnalyzing board with ECS queries...");
                
                self.pcb_world.print_summary();
                self.pcb_world.print_components();
                self.pcb_world.print_mounting_holes();
                
                info!("ECS integration complete");
                println!("\nECS integration complete!");
                self.state = ClientState::Terminated;
            }
            ClientState::Error(err) => {
                warn!("Client encountered an error: {}", err);
                println!("Client encountered an error: {}", err);
                println!("Client is now terminating.");
                self.state = ClientState::Terminated;
            }
            ClientState::Terminated => {
                debug!("Client in terminated state");
                // Final state
            }
        }
    }
}

fn display_banner_message() {
    println!("KiCad ECS Integration Example");
    println!("================================");
    println!("This example demonstrates Entity Component System (ECS) architecture");
    println!("for managing PCB component data from KiCad.\n");
}

/// Main function to run the ECS example
/// 
/// Note how clean the main function is, it simply initializes the supervisor and runs the state machine.
/// This is an artifact of the supervisor pattern, which allows us to encapsulate the state management logic
/// and keep the main function focused on high-level flow control.
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    kicad_ecs::tracing::init_for_examples();
    
    info!("Starting KiCad ECS Integration Example");
    display_banner_message();

    let mut supervisor = Supervisor::new();

    // Run the state machine
    loop {
        supervisor.update().await;

        if supervisor.state == ClientState::Terminated {
            break;
        }

        // Small delay between state transitions for readability
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    info!("Exiting the KiCad ECS client example");
    println!("\nExiting the KiCad ECS client example.");
    Ok(())
}