
#[derive(Debug, Clone)]
pub struct Block {
    pub contents: Vec<u8>,
}

impl Block {
    pub fn new(contents: Vec<u8>) -> Self {
        Block {
            contents,
        }
    }

    pub fn to_block<T>(object: & mut T) -> Self where T: serde::Serialize {
        let encoded: Vec<u8> = bincode::serialize(object).unwrap();
        let block = Block::new(encoded);
        block
    }
}

