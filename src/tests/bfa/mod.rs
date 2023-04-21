#[cfg(test)]
mod bfa_tests {
    use crate::bfa::BFA;
    use std::fs::File;
    use std::io::{Write, Read, Seek, SeekFrom};

    #[test]
    fn test_bfa_put_ok() {
        let mut file = File::create("test_files/test_bfa_put_ok").expect("test_bfa_put_ok failed");
        file.write_all(b"Hello World!").expect("test_bfa_put_ok failed");
        let mut bfa_1 = BFA::new(6, "test_files","test_bfa_put_ok");
        let block_1 = bfa_1.get(0).unwrap();
        let block_2 = bfa_1.get(1).unwrap();
        let a = bfa_1.reserve();
        let b = bfa_1.reserve();
        bfa_1.update(a, block_2).unwrap();
        bfa_1.update(b, block_1).unwrap();

        let mut b = String::new();
        bfa_1.file.seek(SeekFrom::Start(0)).unwrap();
        bfa_1.file.read_to_string(& mut b).unwrap();
        assert_eq!(b, "Hello World!World!Hello ".to_string());
    }

    #[test]
    fn test_bfa_put_fail() {
        let mut file = File::create("test_files/test_bfa_put_fail").expect("test_bfa_put_ok failed");
        file.write_all(b"Hello World!").expect("test_bfa_put_ok failed");
        let mut bfa_1 = BFA::new(6, "test_files", "test_bfa_put_fail");
        let block_1 = bfa_1.get(0).unwrap();
        let block_2 = bfa_1.get(1).unwrap();
        let a = bfa_1.reserve();
        let b = bfa_1.reserve();
        bfa_1.update(a, block_2).unwrap();
        bfa_1.update(b, block_1).unwrap();

        let mut b = String::new();
        bfa_1.file.seek(SeekFrom::Start(0)).unwrap();
        bfa_1.file.read_to_string(& mut b).unwrap();
        assert_ne!(b, "Hello World!Hello World!".to_string());
    }

    #[test]
    fn test_bfa_get_ok() {
        let mut file = File::create("test_files/test_bfa_get_ok").expect("test_bfa_put_ok failed");
        file.write_all(b"Hello World!").expect("test_bfa_put_ok failed");
        let mut bfa_1 = BFA::new(8, "test_files","test_bfa_get_ok");
        let block = bfa_1.get(0).unwrap();
        assert_eq!(block.contents, [72, 101, 108, 108, 111, 32, 87, 111]);
    }

    #[test]
    fn test_bfa_get_fail() {
        let mut file = File::create("test_files/test_bfa_get_fail").expect("test_bfa_put_ok failed");
        file.write_all(b"Hello World!").expect("test_bfa_put_ok failed");
        let mut bfa_1 = BFA::new(8, "test_files","test_bfa_get_fail");
        let block = bfa_1.get(0).unwrap();
        assert_ne!(block.contents, [72, 101, 108, 108, 111, 32, 87, 101]);
    }
}
