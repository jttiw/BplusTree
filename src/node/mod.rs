use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use crate::block::Block;
use crate::inner_element::InnerElement;
use crate::leaf_element::LeafElement;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Node<T: Ord, V: Sized> {
    InnerNode {
        content: Vec<InnerElement<T>>
    },
    LeafNode {
        content: Vec<LeafElement<T, V>>,
        l_ref: Option<usize>,     // -1 if non-existent TODO use Option<usize>
        r_ref: Option<usize>
    },
}

impl <T, V> Node<T, V> where T: Ord + Debug, V: Sized + Debug {

    //Construct from a block (Vector of --Serialized-- Bytes)
    pub fn from_block(block: & mut Block) -> Self where T: DeserializeOwned, V: DeserializeOwned {
        let node = bincode::deserialize(block.contents.as_slice()).expect("error node from block");
        return node
    }

    //Getter and Setter
    pub fn get_inner_content(&mut self) -> Option<&mut Vec<InnerElement<T>>> {
        match self {
            Node::InnerNode {content} => {
                return Some(content);
            }
            Node::LeafNode {content: _, l_ref:_, r_ref:_} => {
                return None;
            }
        }
    }

    pub fn get_leaf_content(&mut self) -> Option<&mut Vec<LeafElement<T, V>>> {
        match self {
            Node::InnerNode { content: _ } => {
                return None;
            }
            Node::LeafNode { content, l_ref: _, r_ref: _ } => {
                return Some(content);
            }
        }
    }

    pub fn get_lref(self) -> Option<usize> {
        match self {
            Node::InnerNode { content: _ } => {
                return None;
            }
            Node::LeafNode { content: _, l_ref, r_ref: _ } => {
                return l_ref.clone();
            }
        }
    }

    pub fn get_rref(&self) -> Option<usize> {
        match self {
            Node::InnerNode { content: _ } => {
                return None;
            }
            Node::LeafNode { content: _, l_ref: _, r_ref } => {
                return r_ref.clone();
            }
        }
    }

    pub fn set_lref(&mut self, id: Option<usize>) {
        match self {
            Node::InnerNode { content: _ } => {}
            Node::LeafNode { content: _, ref mut l_ref, r_ref: _ } => {
                *l_ref = id;
            }
        }
    }

    pub fn set_rref(&mut self, id: Option<usize>) {
        match self {
            Node::InnerNode { content: _ } => {}
            Node::LeafNode { content: _, l_ref: _, ref mut r_ref } => {
                *r_ref = id;
            }
        }
    }

    pub fn set_leaf_content(&mut self, new_content: Vec<LeafElement<T, V>>) {
        match self {
            Node::InnerNode {content: _ } => {
            }
            Node::LeafNode {content, l_ref: _, r_ref: _ } => {
                *content = new_content;
            }
        }
    }
    pub fn set_inner_content(&mut self, new_content: Vec<InnerElement<T>>) {
        match self {
            Node::InnerNode {content} => {
                *content = new_content;
            }
            Node::LeafNode {content: _ , l_ref: _ , r_ref: _ } => {
            }
        }
    }
}
