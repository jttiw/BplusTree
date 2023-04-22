use std::fmt::Debug;
use serde::de::DeserializeOwned;
use crate::b_plus_tree::BTree;
use crate::node::Node;

pub(crate) fn traverse_internal<T :Ord,V>(tree: &mut BTree, given : usize) -> Option<Vec<T>>
    where T: DeserializeOwned + Debug + Copy ,
          V: DeserializeOwned + Debug + Clone {

    let mut result : Vec<T> = Vec::new();
    let tmp: Node<T,V> = Node::from_block(&mut tree.bfa.get(given).expect("from_block error"));
    match tmp {
        Node::LeafNode {content, l_ref: _, r_ref: _} => {
            for i in 0..content.len() {
                result.push(content.get(i).unwrap().comp);
            }
        }
        Node::InnerNode{content} => {
            for i in 0..content.len() {
                result.push(content.get(i).unwrap().comp.clone());
                let mut v :Vec<T> = traverse_internal::<T, V>(tree, content.get(i).unwrap().id).unwrap();
                result.append(&mut v);

            }
        }
    }
    return Some(result);
}