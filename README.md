### Build
```bash
cargo build -r
```
### Run
```bash
$ afptool-rs rk.img ./out
RKFW signature detected
version: 8.1.0
date: 32768-06-20 16:02:06
family: PX30
00000066-0004c1b3 BOOT                       (size: 311630)
0004c1b4-a2e8c9b7 embedded-update.img        (size: 2732853252)
```
```
$ afptool-rs ./out/embedded-update.img  ./out 
Filesize: 2732853252
manufacturer:  RK3326
model: RK3326
00000800-000002eb ./out/package-file
00001000-0004c14e ./out/Image/MiniLoaderAll.bin
0004d800-0000031f ./out/Image/parameter.txt
0004e000-00400000 ./out/Image/trust.img
0044e000-00400000 ./out/Image/uboot.img
0084e000-0000c000 ./out/Image/misc.img
0085a000-001d3c00 ./out/Image/resource.img
00a2e000-012b8814 ./out/Image/kernel.img
01ce7000-0016a40c ./out/Image/boot.img
01e51800-026a07c4 ./out/Image/recovery.img
044f2000-000400ac ./out/Image/cache.img
04532800-4face0f4 ./out/Image/system.img
54001000-0e8eb07c ./out/Image/vendor.img
628ec800-0c82b0a0 ./out/Image/oem.img
6f118000-33d28274 ./out/Image/update_back.img

```
