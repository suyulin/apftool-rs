# apftool-rs

一个用于解包瑞芯微固件映像文件（RKFW 和 RKAF 格式）的 Rust 工具。

## 功能特性

- 解包 RKFW 固件文件
- 提取嵌入式 RKAF 更新映像
- 支持多种瑞芯微芯片系列（RK29xx、RK30xx、RK31xx、RK32xx、RK3368、RK3326、RK3566、PX30）
- 跨平台支持（Windows、macOS、Linux）

## 构建

### 标准构建
```bash
cargo build --release
```

### macOS 通用二进制文件
```bash
./build.sh
```

### GitHub Actions
本项目包含自动化 CI/CD 流程，通过 GitHub Actions 构建以下平台的二进制文件：
- Linux x86_64
- Linux ARM64 (aarch64)
- macOS x86_64
- macOS ARM64 (Apple Silicon)
- macOS 通用二进制文件
- Windows x86_64

当您推送版本标签（例如 `v1.0.0`）时，会自动创建发布版本。

## 使用方法

```bash
afptool-rs <输入文件> <输出目录>
```

### 示例

**解包 RKFW 固件：**
```bash
$ afptool-rs rk.img ./out
RKFW signature detected
version: 8.1.0
family: PX30
00000066-0004c1b3 BOOT                       (size: 311630)
0004c1b4-a2e8c9b7 embedded-update.img        (size: 2732853252)
```

**提取嵌入式 RKAF 更新映像：**
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

## 支持的格式

- **RKFW**：瑞芯微固件包装格式
- **RKAF**：瑞芯微 Android 固件包格式

## 支持的芯片系列

| 芯片代码 | 系列 |
|----------|------|
| 0x50 | rk29xx |
| 0x60 | rk30xx |
| 0x70 | rk31xx |
| 0x80 | rk32xx |
| 0x41 | rk3368 |
| 0x36 | RK3326 |
| 0x38 | RK3566 |
| 0x30 | PX30 |

## 许可证

本项目采用 Apache License 2.0 许可证 - 详见 [LICENSE](LICENSE) 文件。
