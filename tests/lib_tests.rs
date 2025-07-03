#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;
    use afptool_rs::{RKAF_SIGNATURE, RKFW_SIGNATURE, UpdateHeader};

    // 创建模拟的 RKFW 文件用于测试
    fn create_mock_rkfw() -> Vec<u8> {
        let mut data = vec![0u8; 1024];
        
        // 写入 RKFW 签名
        data[0..4].copy_from_slice(RKFW_SIGNATURE);
        
        // 设置版本信息 (8.1.0)
        data[6] = 0;
        data[7] = 0;
        data[8] = 1;
        data[9] = 8;
        
        // 设置芯片类型 (PX30)
        data[0x15] = 0x30;
        
        // 设置引导信息偏移量和大小
        data[0x19] = 0x66;
        data[0x1a] = 0;
        data[0x1b] = 0;
        data[0x1c] = 0;
        
        data[0x1d] = 0x10;
        data[0x1e] = 0;
        data[0x1f] = 0;
        data[0x20] = 0;
        
        // 设置嵌入式更新映像偏移量和大小
        data[0x21] = 0x76;
        data[0x22] = 0;
        data[0x23] = 0;
        data[0x24] = 0;
        
        data[0x25] = 0x20;
        data[0x26] = 0;
        data[0x27] = 0;
        data[0x28] = 0;
        
        // 写入 BOOT 标记
        data[0x66] = b'B';
        data[0x67] = b'O';
        data[0x68] = b'O';
        data[0x69] = b'T';
        
        // 写入 RKAF 标记（模拟嵌入式更新映像）
        data[0x76] = b'R';
        data[0x77] = b'K';
        data[0x78] = b'A';
        data[0x79] = b'F';
        
        data
    }
    
    // 创建模拟的 RKAF 文件用于测试
    fn create_mock_rkaf() -> Vec<u8> {
        let mut data = vec![0u8; 2048];
        
        // 写入 RKAF 签名
        data[0..4].copy_from_slice(RKAF_SIGNATURE);
        
        // 设置长度（文件大小）
        data[4] = 0x00;
        data[5] = 0x08;
        data[6] = 0x00;
        data[7] = 0x00;
        
        // 设置厂商信息 (RK3326)
        let manufacturer = b"RK3326";
        let offset = 4 + 4 + 34 + 30; // magic + length + model + id
        data[offset..offset + manufacturer.len()].copy_from_slice(manufacturer);
        
        // 设置模型信息 (RK3326)
        let model = b"RK3326";
        let offset = 4 + 4; // magic + length
        data[offset..offset + model.len()].copy_from_slice(model);
        
        // 设置分区数量 (0表示没有分区，这样可以避免解包时的错误)
        let num_parts_offset = 4 + 4 + 34 + 30 + 56 + 4 + 4;
        data[num_parts_offset] = 0;
        data[num_parts_offset + 1] = 0;
        data[num_parts_offset + 2] = 0;
        data[num_parts_offset + 3] = 0;
        
        data
    }

    #[test]
    fn test_update_header_from_bytes() {
        let mock_rkaf = create_mock_rkaf();
        let header = UpdateHeader::from_bytes(&mock_rkaf);
        
        assert_eq!(&header.magic, RKAF_SIGNATURE);
        // 使用临时变量避免 packed struct 对齐问题
        let length = header.length;
        assert_eq!(length, 0x800);
        
        // 检查厂商信息
        let manufacturer = b"RK3326\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
        assert_eq!(&header.manufacturer[..], manufacturer);
        
        // 检查型号信息
        let model = b"RK3326\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
        assert_eq!(&header.model[..], model);
        
        // 检查分区数量（使用临时变量避免 packed struct 对齐问题）
        let num_parts = header.num_parts;
        assert_eq!(num_parts, 0);
    }
    
    #[test]
    fn test_update_header_to_bytes() {
        let mut header = UpdateHeader::default();
        header.magic.copy_from_slice(RKAF_SIGNATURE);
        header.length = 0x800;
        header.num_parts = 0;
        
        let manufacturer = b"RK3326";
        header.manufacturer[..manufacturer.len()].copy_from_slice(manufacturer);
        
        let model = b"RK3326";
        header.model[..model.len()].copy_from_slice(model);
        
        let bytes = header.to_bytes();
        assert_eq!(&bytes[0..4], RKAF_SIGNATURE);
        
        // 检查长度
        assert_eq!(bytes[4], 0x00);
        assert_eq!(bytes[5], 0x08);
        assert_eq!(bytes[6], 0x00);
        assert_eq!(bytes[7], 0x00);
        
        // 检查分区数量
        let num_parts_offset = 4 + 4 + 34 + 30 + 56 + 4 + 4;
        assert_eq!(bytes[num_parts_offset], 0);
        assert_eq!(bytes[num_parts_offset + 1], 0);
        assert_eq!(bytes[num_parts_offset + 2], 0);
        assert_eq!(bytes[num_parts_offset + 3], 0);
    }

    #[test]
    fn test_create_mock_files() {
        // 创建测试目录
        let test_dir = Path::new("tests/data/temp");
        if !test_dir.exists() {
            fs::create_dir_all(test_dir).unwrap();
        }
        
        // 创建并写入模拟的 RKFW 文件
        let rkfw_data = create_mock_rkfw();
        let rkfw_path = test_dir.join("mock.rkfw");
        let mut file = File::create(&rkfw_path).unwrap();
        file.write_all(&rkfw_data).unwrap();
        
        assert!(rkfw_path.exists());
        assert_eq!(fs::metadata(&rkfw_path).unwrap().len(), rkfw_data.len() as u64);
        
        // 创建并写入模拟的 RKAF 文件
        let rkaf_data = create_mock_rkaf();
        let rkaf_path = test_dir.join("mock.rkaf");
        let mut file = File::create(&rkaf_path).unwrap();
        file.write_all(&rkaf_data).unwrap();
        
        assert!(rkaf_path.exists());
        assert_eq!(fs::metadata(&rkaf_path).unwrap().len(), rkaf_data.len() as u64);
        
        // 清理测试文件
        fs::remove_file(&rkfw_path).unwrap();
        fs::remove_file(&rkaf_path).unwrap();
    }
}
