use clap::Parser;
use afptool_rs::unpack_file;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "afptool-rs")]
#[command(about = "A Rust tool for unpacking RockChip firmware images")]
#[command(version)]
struct Args {
    /// Input firmware file path
    #[arg(help = "Path to the firmware file (RKFW or RKAF format)")]
    input: String,
    
    /// Output directory path
    #[arg(help = "Directory where extracted files will be saved")]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    unpack_file(&args.input, &args.output)?;
    Ok(())
}