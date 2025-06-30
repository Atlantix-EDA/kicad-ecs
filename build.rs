use std::io::Result;

fn main() -> Result<()> {
    // Only regenerate if proto files change
    println!("cargo:rerun-if-changed=proto/");
    
    let proto_files = [
        "proto/common/envelope.proto",
        "proto/common/types.proto", 
        "proto/board/board_types.proto",
        "proto/board/board_commands.proto",
        // Add more as needed
    ];
    
    prost_build::Config::new()
        .out_dir("src/generated")
        // Optional: customize generation
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&proto_files, &["proto/"])?;
        
    Ok(())
}