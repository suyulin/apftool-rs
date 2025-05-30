# apftool-rs

A Rust tool for unpacking RockChip firmware images (RKFW and RKAF formats).

## Features

- Unpack RKFW firmware files
- Extract embedded RKAF update images
- Support for various RockChip chip families (RK29xx, RK30xx, RK31xx, RK32xx, RK3368, RK3326, RK3566, PX30)
- Cross-platform support (Windows, macOS, Linux)

## Build

### Standard build
```bash
cargo build --release
```

### Universal macOS binary
```bash
./build.sh
```

## Usage

```bash
afptool-rs <input_file> <output_directory>
```

### Examples

**Unpack RKFW firmware:**
```bash
$ afptool-rs rk.img ./out
RKFW signature detected
version: 8.1.0
family: PX30
00000066-0004c1b3 BOOT                       (size: 311630)
0004c1b4-a2e8c9b7 embedded-update.img        (size: 2732853252)
```

**Extract embedded RKAF update image:**
```bash
$ afptool-rs ./out/embedded-update.img ./out 
Filesize: 2732853252
manufacturer: RK3326
model: RK3326
00000800-000002eb ./out/package-file
00001000-0004c14e ./out/Image/MiniLoaderAll.bin
0004d800-0000031f ./out/Image/parameter.txt
0004e000-00400000 ./out/Image/trust.img
0084e000-0000c000 ./out/Image/misc.img
00a2e000-012b8814 ./out/Image/kernel.img
01ce7000-0016a40c ./out/Image/boot.img
01e51800-026a07c4 ./out/Image/recovery.img
628ec800-0c82b0a0 ./out/Image/oem.img
6f118000-33d28274 ./out/Image/update_back.img
```

## Supported Formats

- **RKFW**: RockChip firmware wrapper format
- **RKAF**: RockChip Android firmware package format

## Supported Chip Families

| Chip Code | Family |
|-----------|--------|
| 0x50 | rk29xx |
| 0x60 | rk30xx |
| 0x70 | rk31xx |
| 0x80 | rk32xx |
| 0x41 | rk3368 |
| 0x36 | RK3326 |
| 0x38 | RK3566 |
| 0x30 | PX30 |

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
