use std::{mem, str};
use std::fs::{create_dir_all, File};
use std::io::{Read, Seek, Write};
use std::path::Path;
use anyhow::{anyhow, Result};

pub const RKAFP_MAGIC: &str = "RKAF";
pub const PARM_MAGIC: &str = "PARM";
pub const MAX_PARTS: usize = 16;
pub const MAX_NAME_LEN: usize = 32;
const MAX_FULL_PATH_LEN: usize = 60;
const MAX_MODEL_LEN: usize = 34;
const MAX_ID_LEN: usize = 30;
const MAX_MANUFACTURER_LEN: usize = 56;
pub const RKAF_SIGNATURE: &[u8] = b"RKAF";
pub const RKFW_SIGNATURE: &[u8] = b"RKFW";
pub const RKFP_SIGNATURE: &[u8] = b"RKFP";

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct UpdatePart {
    name: [u8; MAX_NAME_LEN],
    pub full_path: [u8; MAX_FULL_PATH_LEN],
    flash_size: u32,
    pub part_offset: u32,
    flash_offset: u32,
    padded_size: u32,
    pub part_byte_count: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct UpdateHeader {
    pub magic: [u8; 4],
    pub length: u32,
    pub model: [u8; MAX_MODEL_LEN],
    id: [u8; MAX_ID_LEN],
    pub manufacturer: [u8; MAX_MANUFACTURER_LEN],
    unknown1: u32,
    version: u32,
    pub num_parts: u32,
    pub parts: [UpdatePart; MAX_PARTS],
    reserved: [u8; 116],
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct ParamHeader {
    magic: [u8; 4],
    length: u32,
}


impl UpdateHeader {
    pub fn default() -> Self {
        Self {
            magic: [0u8; 4],
            length: 0,
            model: [0u8; MAX_MODEL_LEN],
            id: [0u8; MAX_ID_LEN],
            manufacturer: [0u8; MAX_MANUFACTURER_LEN],
            unknown1: 0,
            version: 0,
            num_parts: 0,
            parts: [UpdatePart::default(); MAX_PARTS],
            reserved: [0u8; 116],

        }
    }
    pub fn from_bytes(bytes: &[u8]) -> &UpdateHeader {
        unsafe { mem::transmute(bytes.as_ptr()) }
    }

    pub fn to_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const _ as *const u8, mem::size_of::<UpdateHeader>()) }
    }
}

impl UpdatePart {
    pub fn default() -> Self {
        Self {
            name: [0u8; MAX_NAME_LEN],
            full_path: [0u8; MAX_FULL_PATH_LEN],
            flash_size: 0,
            part_offset: 0,
            flash_offset: 0,
            padded_size: 0,
            part_byte_count: 0,
        }
    }
}

pub fn unpack_file(file_path: &str, dst_path: &str) -> Result<()> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let signature = &buffer[0..4];
    match signature {
        RKAF_SIGNATURE => unpack_rkafp(file_path, dst_path)?,
        RKFW_SIGNATURE => unpack_rkfw(&buffer, dst_path)?,
        _ => {
            return Err(anyhow!("Unknown signature: {:?}", signature));
        }
    }
    Ok(())
}

fn unpack_rkfw(buf: &[u8], dst_path: &str) -> Result<()> {
    let mut chip: Option<&str> = None;

    println!("RKFW signature detected");
    println!(
        "version: {}.{}.{}",
        buf[9],
        buf[8],
        ((buf[7] as u16) << 8) + buf[6] as u16
    );
    // println!(
    //     "date: {}-{:02}-{:02} {:02}:{:02}:{:02}",
    //     (buf[0x0f] as u16) << 8 + buf[0x0e] as u16,
    //     buf[0x10],
    //     buf[0x11],
    //     buf[0x12],
    //     buf[0x13],
    //     buf[0x14]
    // );

    match buf[0x15] {
        0x50 => chip = Some("rk29xx"),
        0x60 => chip = Some("rk30xx"),
        0x70 => chip = Some("rk31xx"),
        0x80 => chip = Some("rk32xx"),
        0x41 => chip = Some("rk3368"),
        0x36 => chip = Some("RK3326"),
        0x38 => chip = Some("RK3566"),
        0x30 => chip = Some("PX30"),
        _ => println!(
            "You got a brand new chip ({:#x}), congratulations!!!",
            buf[0x15]
        ),
    }

    println!("family: {}", chip.unwrap_or("unknown"));

    let ioff = get_u32_le(&buf[0x19..]);
    let isize: u32 = get_u32_le(&buf[0x1d..]);

    // if &buf[ioff as usize..ioff as usize + 4] != b"BOOT" {
    //     panic!("cannot find BOOT signature");
    // }

    println!(
        "{:08x}-{:08x} {:26} (size: {})",
        ioff,
        ioff + isize - 1,
        "BOOT",
        isize
    );
    create_dir_all(dst_path)?;
    write_file(
        &Path::new(&format!("{}/BOOT", dst_path)),
        &buf[ioff as usize..ioff as usize + (isize as usize)],
    )?;

    let ioff = get_u32_le(&buf[0x21..]);
    let isize = get_u32_le(&buf[0x25..]);

    if &buf[ioff as usize..ioff as usize + 4] != b"RKAF" {
        panic!("cannot find embedded RKAF update.img");
    }

    println!(
        "{:08x}-{:08x} {:26} (size: {})",
        ioff,
        ioff + isize - 1,
        "embedded-update.img",
        isize
    );
    write_file(
        &Path::new(&format!("{}/embedded-update.img", dst_path)),
        &buf[ioff as usize..ioff as usize + isize as usize],
    )?;
    Ok(())
}

pub unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    core::slice::from_raw_parts(
        (p as *const T) as *const u8,
        mem::size_of::<T>(),
    )
}

pub fn info_and_fatal(is_fatal: bool, message: String) {
    if is_fatal {
        eprint!("rkunpack: fatal: ");
    } else {
        eprint!("rkunpack: info: ");
    }
    eprintln!("{}", message);
    if is_fatal {
        std::process::exit(1);
    }
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        info_and_fatal(false, format!($($arg)*));
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        info_and_fatal(true, format!($($arg)*));
    };
}
fn extract_file(fp: &mut File, offset: u64, len: u64, full_path: &str) -> Result<()> {
    println!("{:08x}-{:08x} {}", offset, len, full_path);
    let mut buffer = vec![0u8; 16 * 1024];
    let mut fp_out = File::create(full_path)?;

    fp.seek(std::io::SeekFrom::Start(offset))?;

    let mut remaining = len;

    while remaining > 0 {
        let read_len = std::cmp::min(remaining as usize, buffer.len());
        let read_bytes = fp.read(&mut buffer[..read_len])?;

        if read_bytes != read_len {
            return Err(anyhow!("Insufficient length in container image file"));
        }

        fp_out.write_all(&buffer[..read_len])?;

        remaining -= read_len as u64;
    }

    Ok(())
}

fn unpack_rkafp(file_path: &str, dst_path: &str) -> Result<()> {
    let mut fp = File::open(file_path)?;
    let mut buf = vec![0u8; mem::size_of::<UpdateHeader>()];
    fp.read_exact(&mut buf)?;
    let header = UpdateHeader::from_bytes(buf.as_mut());
    let magic_str = str::from_utf8(&header.magic)?;
    if magic_str != RKAFP_MAGIC {
        return Err(anyhow!("Invalid header magic id"));
    }

    let filesize = fp.metadata()?.len();
    println!("Filesize: {}", filesize);
    if filesize - 4 != header.length as u64 {
        eprintln!("update_header.length cannot be correct, cannot check CRC");
    }
    create_dir_all(format!("{}/Image", dst_path))?;
    // 安全地从null-terminated字符串中提取文本
    let manufacturer = std::ffi::CStr::from_bytes_until_nul(&header.manufacturer)
        .map(|s| s.to_string_lossy())
        .unwrap_or_else(|_| "unknown".into());
    let model = std::ffi::CStr::from_bytes_until_nul(&header.model)
        .map(|s| s.to_string_lossy())
        .unwrap_or_else(|_| "unknown".into());
    
    println!("manufacturer: {}", manufacturer);
    println!("model: {}", model);
    for i in 0..header.num_parts {
        let part = &header.parts[i as usize];
        // 安全地提取路径字符串
        if let Ok(cstr) = std::ffi::CStr::from_bytes_until_nul(&part.full_path) {
            let part_full_path = cstr.to_string_lossy();
            if part_full_path == "SELF" || part_full_path == "RESERVED" {
                continue;
            }
            let part_full_path = format!("{}/{}", dst_path, part_full_path);
            extract_file(
                &mut fp,
                part.part_offset as u64,
                part.part_byte_count as u64,
                &part_full_path,
            )?;
        }
    }

    Ok(())
}


fn get_u32_le(slice: &[u8]) -> u32 {
    u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]])
}

fn write_file(path: &Path, buffer: &[u8]) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(buffer)?;
    Ok(())
}
