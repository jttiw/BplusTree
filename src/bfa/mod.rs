use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use crate::block::Block;
use std::collections::HashMap;
use std::error::Error;


//Block File Access
pub struct BFA {
    pub block_size: usize,
    pub file: File,
    pub update_file: Vec<bool>,
    pub reserve_map: HashMap<usize, bool>,
    pub metadata_file: HashMap<String, String>,
    pub reservecount: usize,
}

impl BFA {
    pub fn new(block_size: usize, dir: &str, filestr: &str ) -> BFA {
        //setting up file paths
        let filepath = format!("{}\\{}", dir, filestr);
        let updatepath = format!("{}\\{}update", dir, filestr);
        let metadatapath = format!("{}\\{}metadata", dir, filestr);
        //read and write permissions
        match OpenOptions::new()
            .read(true)
            .write(true)
            .open(&filepath) {

            //file already exists
            Ok(file) =>{
                let mut update_file= Vec::new();
                let update = File::open(& updatepath);
                match update {
                    Ok(up) => {
                        for bool in up.bytes() {
                            match bool {
                                Ok(byte) => {
                                    if byte == 49 { update_file.push(true);}
                                    else { update_file.push(false);}
                                },
                                _ => {println!("updatefile corrupted")}
                            }
                        }
                    },
                    _ => {println!("updatefile does not exist")}
                };

                if update_file.len() == 0 {
                    update_file = vec![true; file.metadata().unwrap().len() as usize / block_size];
                    println!("creating new updatefile");
                    println!("filesize: {}", file.metadata().unwrap().len() as usize);
                    println!("blocksize: {}", block_size);
                    println!("{:?}", update_file);

                }
                let mut metadata_file;

                match OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&metadatapath) {
                    Ok(mut met) => {
                        let mut data = Vec::new();
                        met.read_to_end(&mut data).expect("error reading metadata to [u8]");
                        metadata_file = bincode::deserialize(&data).expect("error parsing");
                    }
                    _ => {
                        metadata_file = HashMap::new();
                        metadata_file.insert("root".to_string(), "0".to_string());
                        metadata_file.insert("updatepath".to_string(), updatepath);
                        metadata_file.insert("metadatapath".to_string(), metadatapath);
                    }
                }

                let reservecount = update_file.len();

                BFA {
                    block_size,
                    file,
                    update_file,
                    reserve_map: HashMap::new(),
                    metadata_file,
                    reservecount,
                }
            },

            //new file
            _ => {
                let mut file = File::create(&filepath).expect("file creation failed");

                file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&filepath)
                    .expect("Unable to open file.");

                let mut metadata_file = HashMap::new();

                metadata_file.insert("root".to_string(), "0".to_string());
                metadata_file.insert("updatepath".to_string(), updatepath);
                metadata_file.insert("metadatapath".to_string(), metadatapath);

                BFA {
                    block_size,
                    file,
                    update_file: Vec::new(),
                    reserve_map: HashMap::new(),
                    metadata_file,
                    reservecount: 0,
                }

            }
        }
    }

    //get bytes from index id to a block
    pub fn get(&mut self, id: usize) -> Option<Block>{
        if self.update_file[id] {
            let mut vec:Vec<u8> = vec![0; self.block_size];
            self.file.seek(SeekFrom::Start((id * self.block_size) as u64)).expect("error bfa get");
            self.file.read(& mut vec).expect("error bfa get");
            let block = Block::new(vec);
            Some(block)
        }
        else {
            None
        }

    }

    //put content of a block into file at index id, if id is reserved or already written
    pub fn update(&mut self, id: usize, mut block: Block) -> Result<(), Box<dyn Error>> {
        if block.contents.len() > self.block_size {
            return Err("Block is too large".into());
        }
        else {
            let res = self.reserve_map.get(&id);
            //fill entire block, important for the last block
            if block.contents.len() < self.block_size {
                for _i in block.contents.len()..self.block_size {
                    block.contents.push(0);
                }
            }

            match res {
                Some(bool) => {
                    if bool == &true {
                        self.file.seek(SeekFrom::Start((id * self.block_size) as u64)).expect("error bfa update");
                        self.file.write(&block.contents)?;
                        self.update_file.insert(id,true);
                        self.reserve_map.remove(&id);
                    }
                    else {
                        return Err("id not reserved".into());
                    }
                }
                None => {
                    if self.update_file[id] {
                        self.file.seek(SeekFrom::Start((id * self.block_size) as u64)).expect("error bfa update");
                        self.file.write(&block.contents)?;
                        self.reserve_map.remove(&id);
                    }
                    else {
                        return Err("id not reserved".into());
                    }
                }
            }
            return Ok(())
        }
    }


    pub fn get_file_size(&self) -> usize {
        self.file.metadata().unwrap().len() as usize
    }

    pub fn get_block_count(&self) -> usize {
        self.get_file_size() / self.get_block_size() + 1
    }

    pub fn get_block_size(& self) -> usize {
        self.block_size
    }

    pub fn reserve(& mut self) -> usize {
        self.reserve_map.insert(self.reservecount, true);
        self.reservecount += 1;
        self.reservecount - 1
    }

    pub fn contains(& self, id: usize) -> bool {
        if id > self.update_file.len() {
            println!("id too large");
            false
        }
        else {self.update_file[id]}
    }

    pub fn remove(& mut self, id: usize) {
        if id > self.update_file.len() {
            println!("id: {} too large", id);
        }
        else {
            self.update_file[id] = false;
        }
    }

    //creates updatefile and metadata
    pub fn close(& mut self) -> Result<(), Box<dyn Error>> {
        self.file.flush().expect("error bfa close");
        self.reserve_map.clear();
        File::create(self.metadata_file.get("updatepath").expect("updatepath does not exist")).expect("Updatefile creation failed");
        let mut file_1 = OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.metadata_file.get("updatepath").expect("updatepath does not exist"))
            .expect("Unable to open file.");
        for i in 0 .. self.update_file.len() {
            if self.update_file[i] { write!(file_1, "1" ).expect("error bfa close");}
            else {write!(file_1, "0").expect("error bfa close"); }
        };
        File::create(self.metadata_file.get("metadatapath").expect("metadatapath does not exist")).expect("Metadatafile creation failed");
        let mut file_2 = OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.metadata_file.get("metadatapath").expect("metadatapath does not exist"))
            .expect("Unable to open file.");
        let encoded = bincode::serialize(&self.metadata_file).unwrap();
        file_2.write(&encoded).expect("Error writing metadata");
        Ok(())
    }

    pub fn insert(& mut self, block: Block)  {
        let id = self.reserve();
        self.update(id, block).expect("error bfa insert");
    }

    pub fn set_metadata(& mut self,  key: String, value: usize) {
        self.metadata_file.insert(key, format!("{}", value));
    }

    pub fn set_root(&mut self, root: usize) {
        self.metadata_file.insert("root".to_string(), root.to_string());
    }

    pub fn get_root(&mut self) -> usize {
        let root_string = self.metadata_file.get("root").expect("no such string");
        let root = root_string.parse::<usize>().expect("invalid root_string");
        return root;
    }

    pub fn blocks(mut self) -> Vec<Block>{
        let mut vec: Vec<Block> = Vec::new();
        for i in 0..self.update_file.len() {
            let block = self.get(i).expect("error getting blocks").clone();
            vec.push(block)
        }
        return vec;
    }

}


#[cfg(test)]
mod tests {
    use crate::bfa::BFA;
    use std::fs::File;
    use std::io::{Write, Read, Seek, SeekFrom};

    #[test]
    fn test_bfa_put_ok() {
        let mut file = File::create("tests\\test_bfa_put_ok").expect("test_bfa_put_ok failed");
        file.write_all(b"Hello World!").expect("test_bfa_put_ok failed");
        let mut bfa_1 = BFA::new(6, "tests","test_bfa_put_ok");
        let block_1 = bfa_1.get(0).unwrap();
        let block_2 = bfa_1.get(1).unwrap();
        let a = bfa_1.reserve();
        let b = bfa_1.reserve();
        bfa_1.update(a, block_2);
        bfa_1.update(b, block_1);

        let mut b = String::new();
        bfa_1.file.seek(SeekFrom::Start(0));
        bfa_1.file.read_to_string(& mut b);
        assert_eq!(b, "Hello World!World!Hello ".to_string());
    }

    #[test]
    fn test_bfa_put_fail() {
        let mut file = File::create("tests\\test_bfa_put_fail").expect("test_bfa_put_ok failed");
        file.write_all(b"Hello World!").expect("test_bfa_put_ok failed");
        let mut bfa_1 = BFA::new(6, "tests", "test_bfa_put_fail");
        let block_1 = bfa_1.get(0).unwrap();
        let block_2 = bfa_1.get(1).unwrap();
        let a = bfa_1.reserve();
        let b = bfa_1.reserve();
        bfa_1.update(a, block_2);
        bfa_1.update(b, block_1);

        let mut b = String::new();
        bfa_1.file.seek(SeekFrom::Start(0));
        bfa_1.file.read_to_string(& mut b);
        assert_ne!(b, "Hello World!Hello World!".to_string());
    }

    #[test]
    fn test_bfa_get_ok() {
        let mut file = File::create("tests\\test_bfa_get_ok").expect("test_bfa_put_ok failed");
        file.write_all(b"Hello World!").expect("test_bfa_put_ok failed");
        let mut bfa_1 = BFA::new(8, "tests","test_bfa_get_ok");
        let mut block = bfa_1.get(0).unwrap();
        assert_eq!(block.contents, [72, 101, 108, 108, 111, 32, 87, 111]);
    }

    #[test]
    fn test_bfa_get_fail() {
        let mut file = File::create("tests\\test_bfa_get_fail").expect("test_bfa_put_ok failed");
        file.write_all(b"Hello World!").expect("test_bfa_put_ok failed");
        let mut bfa_1 = BFA::new(8, "tests","test_bfa_get_fail");
        let mut block = bfa_1.get(0).unwrap();
        assert_ne!(block.contents, [72, 101, 108, 108, 111, 32, 87, 101]);
    }
}
