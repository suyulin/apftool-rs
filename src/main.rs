use std::env;
use afptool_rs::unpack_file;
use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: afptool-rs <src_dir> <out_dir>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let dst_path = &args[2];
    unpack_file(file_path, dst_path)?;
    Ok(())
}



