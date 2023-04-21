use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use crate::block::Block;
use std::collections::HashMap;
use std::error::Error;


//Block File Access
pub struct BFA {
    pub block_size: usize,
    pub file: File,
    pub update_flags: Vec<bool>,
    pub reserve_map: HashMap<usize, bool>,
    pub metadata_file: HashMap<String, String>,
    pub reserve_count: usize,
}

impl BFA {
    pub fn new(block_size: usize, dir: &str, filestr: &str ) -> BFA {
        //setting up file paths
        let filepath = format!("{}/{}", dir, filestr);
        let updatepath = format!("{}/{}_update", dir, filestr);
        let metadatapath = format!("{}/{}_metadata", dir, filestr);

        //read and write permissions
        match OpenOptions::new()
            .read(true)
            .write(true)
            .open(&filepath) {

            //file already exists -> read contents into vector
            Ok(file) =>{
                let mut update_file_contents = Vec::new();
                let update = File::open(& updatepath);
                match update {
                    Ok(update_file) => {
                        for bool in update_file.bytes() {
                            match bool {
                                Ok(byte) => {
                                    if byte == 49 { update_file_contents.push(true);}
                                    else { update_file_contents.push(false);}
                                },
                                _ => {println!("updatefile corrupted")}
                            }
                        }
                    }
                    _ => {println!("updatefile does not exist")}
                };

                if update_file_contents.len() == 0 {
                    update_file_contents = vec![true; file.metadata().unwrap().len() as usize / block_size];
                    println!("creating new updatefile");
                    println!("filesize: {}", file.metadata().unwrap().len() as usize);
                    println!("blocksize: {}", block_size);
                    println!("{:?}", update_file_contents);

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
                        metadata_file.insert("root".to_string(), "0".to_string());  //Todo maybe set root to variable? Better const
                        metadata_file.insert("updatepath".to_string(), updatepath);
                        metadata_file.insert("metadatapath".to_string(), metadatapath);
                    }
                }

                let reservecount = update_file_contents.len();

                BFA {
                    block_size,
                    file,
                    update_flags: update_file_contents,
                    reserve_map: HashMap::new(),
                    metadata_file,
                    reserve_count: reservecount,
                }
            },

            //new file
            _ => {
                #[allow(unused_assignments)]
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
                    update_flags: Vec::new(),
                    reserve_map: HashMap::new(),
                    metadata_file,
                    reserve_count: 0,
                }

            }
        }
    }

    //get the id-th Block in the File
    pub fn get(&mut self, id: usize) -> Option<Block>{
        if self.update_flags[id] {
            let mut buffer:Vec<u8> = vec![0; self.block_size];
            self.file.seek(SeekFrom::Start((id * self.block_size) as u64)).expect("error bfa get");
            self.file.read(& mut buffer).expect("error bfa get");
            let block = Block::new(buffer);
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
            if block.contents.len() < self.block_size {
                //Pad with 0s
                for _i in block.contents.len()..self.block_size {
                    block.contents.push(0);
                }
            }
            //check if block in file is reserved
            match self.reserve_map.get(&id) {
                Some(bool) => {
                    if bool == &true {
                        self.file.seek(SeekFrom::Start((id * self.block_size) as u64)).expect("error bfa update");
                        self.file.write(&block.contents)?;
                        self.update_flags.insert(id, true);
                        self.reserve_map.remove(&id);
                    }
                    else {
                        return Err("id not reserved".into());
                    }
                }
                None => {
                    if self.update_flags[id] {
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
        self.reserve_map.insert(self.reserve_count, true);
        self.reserve_count += 1;
        self.reserve_count - 1
    }

    pub fn contains(& self, id: usize) -> bool {
        if id > self.update_flags.len() {
            println!("id too large");
            false
        }
        else {self.update_flags[id]}
    }

    pub fn remove(& mut self, id: usize) {
        if id > self.update_flags.len() {
            println!("id: {} too large", id);
        }
        else {
            self.update_flags[id] = false;
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
        for i in 0 .. self.update_flags.len() {
            if self.update_flags[i] { write!(file_1, "1" ).expect("error bfa close");}
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

    pub fn blocks(&mut self) -> Vec<Block>{
        let mut vec: Vec<Block> = Vec::new();
        for i in 0..self.update_flags.len() {
            let block = self.get(i).expect("error getting blocks").clone();
            vec.push(block)
        }
        return vec;
    }

}