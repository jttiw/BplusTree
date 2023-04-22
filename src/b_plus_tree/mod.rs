mod search;
mod insert;
mod split;
mod interval_search;
mod traverse;
mod delete;
mod print;
mod fill;
mod join;
mod fix_parent;

use crate::node::Node;
use crate::inner_element::InnerElement;
use crate::leaf_element::LeafElement;
use crate::block::Block;
use crate::bfa::BFA;



use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::{Debug, Error};
use std::mem::size_of_val;


pub struct BTree {
    pub root_id:usize,
    pub bfa:BFA,
    pub max:usize,  //maximum Number of Elements per Node -> is calculated by Blocksize / size of key-value datatype (in insert func)
}

impl BTree {

    //constructor- creates BFA
    pub fn new(block_size: usize, dir: &str, filestr: &str) -> BTree {
        let bfa = BFA::new(block_size, dir, filestr);
        BTree{
            root_id: 0,
            bfa,
            max: 0
        }
    }

    //gets information like max and root from metafile
    pub fn set_up(&mut self) {
        self.root_id = self.get_root();
        let max = self.get_max();
        match max {
            Some(string) => {
                let max = string.parse::<usize>().expect("invalid max_string");
                self.max = max;
            }
            None => {
                self.max = 0;   //unreachable?
            }
        }
    }

    pub fn set_root(&mut self, root: usize) {
        self.root_id = root;
        self.bfa.metadata_file.insert("root".to_string(), root.to_string());
    }

    pub fn set_max(&mut self, max: usize) {
        self.max = max;
        self.bfa.metadata_file.insert("max".to_string(), max.to_string());
    }

    pub fn get_root(&mut self) -> usize {
        let root_string = self.bfa.metadata_file.get("root").expect("no such string");
        let root = root_string.parse::<usize>().expect("invalid root_string");
        return root;
    }

    pub fn get_max(&mut self) -> Option<&String> {
        let max_string = self.bfa.metadata_file.get("max");
        return max_string;
    }

    pub fn close(&mut self) {
        self.set_root(self.root_id);    //update root in metadatafile
        self.set_max(self.max); // is this needed? may does not change
        self.bfa.close().expect("error btree close");
    }

    //fügt neuen comp in Node mit id = id ein
    fn adjust_ref<T, V> (&mut self, comp: T, id: usize, _data: V)
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {

        let mut tmp: Node<T, V> = Node::from_block(&mut self.bfa.get(id).unwrap());
        for i in 0..tmp.get_inner_content().unwrap().len(){
            if &comp < &tmp.get_inner_content().unwrap().get(i).unwrap().comp {
                tmp.get_inner_content().unwrap()[i].comp = comp;
                break;
            }
        }
        let block = Block::to_block(&mut tmp);
        self.bfa.update(id,block).expect("error btree adjust ref");
    }

    fn new_root<T, V> (&mut self, el1: InnerElement<T>, el2: InnerElement<T>, _data: V, current_id: usize)
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {
        if current_id != self.root_id {
            eprintln!("passed[0] is not root !? Something went wrong");
            panic!();
            //return;
        }
        let mut new_root_node: Node<T, V> = Node::InnerNode{content: vec![el1, el2]};
        let root_id = self.bfa.reserve();
        self.set_root(root_id);
        let block = Block::to_block(&mut new_root_node);
        self.bfa.update(self.root_id, block).expect("error btree new root");
    }

    fn insert_into_node<T, V> (&mut self, el : InnerElement<T>, id : usize, _data: V) -> bool
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized {
        let tmp: Node<T, V> = Node::from_block(&mut self.bfa.get(id).unwrap());
        match tmp {
            Node::InnerNode {mut content} => {
                if content.len() < self.max {
                    for i in 0..content.len() {
                        if el.comp < content.get(i).unwrap().comp {
                            content.insert(i, el);
                            break;
                        }
                        if i == content.len() - 1 {
                            content.push(el);
                        }
                    }
                    let mut new_tmp: Node<T, V> = Node::InnerNode {content};
                    let block = Block::to_block(&mut new_tmp);
                    self.bfa.update(id,block).expect("error btree insert into Node");
                    return true
                }
                else {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
    }

    //***FUNCTIONALITY***

    pub fn search<T :Ord, V>(&mut self, x: T) -> Option<V>
        where T: DeserializeOwned + Debug,
              V: Serialize + DeserializeOwned + Debug {

        let data = search::search_internal(self,x, self.root_id);
        return data;
    }

    pub fn interval_search<T :Ord, V> (&mut self, start: T, end: T) -> Option<Vec<V>>
        where T: DeserializeOwned + Debug,
              V: Serialize + DeserializeOwned + Debug + Clone {
        if end < start {
            return None;
        }
        else {
            let vec = interval_search::interval_search_internal(self, start, end, self.root_id, Vec::new());
            return vec;
        }
    }


    // Traverse Tree in pre-Order
    // Maybe in the future (starting from @param given Node, writing all traversed Comparators to a Vector) -> this is buggy -> For now, always start with root_id
    pub fn traverse<T :Ord,V>(&mut self) -> Option<Vec<T>>
        where T: DeserializeOwned + Debug + Copy ,
              V: DeserializeOwned + Debug + Clone {

        traverse::traverse_internal::<T,V>(self, self.root_id)
    }


    pub fn insert<T, V>(&mut self, key: T, value: V) -> Result<(), Error>   //TODO dont kill program on error
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {

        self.check_if_first_insert(&key,&value);

        let search: Option<V> = search::search_internal(self,key, self.root_id);
        match search {
            Some(_val) => {
                println!("comp {:?} already exists", &key);
                return Err(Error);
            }
            None => {
                let element = LeafElement::new(key, value);
                let res = insert::b_tree_insert(self, element, self.root_id, vec![self.root_id]);
                return res;
            }
        }
    }

    //Print the Structure of Tree for Debugging -> starting_point can be any existing id
    // Turbofish notation
    // Give a Node id Some(id) as arbitrary start Point or None to start at root
    pub fn print<T, V>(&mut self, _starting_point: Option<T>)
            where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
                  V: Serialize + DeserializeOwned + Debug + Sized + Clone {
        /*TODO Idea:
        match starting_point {
            Some(id)=>self.print_tree_structure::<T, V>(id),
            None=>self.print_tree_structure::<T, V>(self.root_id)
        }
        TODO but root_id is usize not T and is initialized from metadata file => parse from metadata with match type { "usize"=> parse::<usize> ...
        */
        print::print_tree_structure::<T, V>(self, self.root_id);
        println!();
    }

    pub fn delete<T, V> (&mut self, x: T) -> Option<V>
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {

        let data = delete::delete_internal(self, x, self.root_id, Vec::new());
        return data;
    }



    //checks if insert into empty tree -> calcs max and creates empty node
    pub fn check_if_first_insert<T, V>(&mut self, key: &T, value: &V)
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
        V: Serialize + DeserializeOwned + Debug + Sized + Clone {

        if self.bfa.reserve_count == 0 && self.max == 0 {   // First insert into empty Tree
            //Calc max based on blocksize and nodesize
            //bincode needs ~32 bytes for the information from leafNode and 16 bytes for the calculation of the sizes of comp and value TODO check
            let value_size = size_of_val(value);
            let key_size = size_of_val(key);
            let max = (self.bfa.block_size - 32) / (value_size + key_size + 16);
            self.set_max(max);
            let mut node: Node<T, V> = Node::LeafNode{content: Vec::new(), l_ref: None, r_ref: None};
            let block = Block::to_block(&mut node);
            self.bfa.insert(block);
        }
    }
}




