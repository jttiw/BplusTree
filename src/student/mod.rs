use serde_derive::*;
use crate::block::Block;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Student {
    pub name: String,
    pub vorname: String,
    pub matrikelnr: usize,
}

impl Student {
    pub fn new(name: &str, vorname: &str, matrikelnr: usize) -> Self {
        Self {
            name: name.to_string(),
            vorname: vorname.to_string(),
            matrikelnr
        }
    }

    pub fn from_block(block: & mut Block) -> Self {
        println!("{:?}", block.contents);
        let back = bincode::deserialize(block.contents.as_slice()).unwrap();
        back
    }

}

#[derive(Debug, Deserialize, Serialize)]
pub struct StudentList {
    liste: Vec<Student>
}

impl StudentList {
    pub fn new() -> Self {
        Self {
            liste: Vec::new(),
        }
    }

    pub fn add(& mut self, student: Student) {
        self.liste.push(student);
    }
     pub fn from_block(block: & mut Block) -> Self {
         println!("{:?}", block.contents);
         let back = bincode::deserialize(block.contents.as_slice()).unwrap();
         back
     }
}

