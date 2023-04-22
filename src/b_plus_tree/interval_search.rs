use std::fmt::Debug;
use serde::de::DeserializeOwned;
use crate::b_plus_tree::BTree;
use crate::node::Node;

pub(crate) fn interval_search_internal<T, V> (tree: &mut BTree, start: T, end: T, given: usize, mut result: Vec<V>) -> Option<Vec<V>>
    where T: DeserializeOwned + Debug + Ord,
          V: DeserializeOwned + Debug + Sized + Clone {

    let tmp: Node<T, V> = Node::from_block(&mut tree.bfa.get(given).expect("error search internal"));
    match tmp {
        Node::LeafNode {content, l_ref: _, r_ref} => {
            for i in 0..content.len() {
                let element = content.get(i).expect("error search internal");
                if element.comp >= start && element.comp <= end {
                    result.push(element.data.clone());
                }
            }
            if content.last().unwrap().comp <= end {
                match r_ref {
                    None=> return Some(result),
                    Some(pointer)=> return interval_search_internal(tree, start, end, pointer, result)
                }
            }
            else {
                return Some(result);
            }
        }
        Node::InnerNode{content} => {
            for i in content {
                if i.comp >= start {
                    return interval_search_internal(tree, start, end, i.id, result);
                }
            }
        }
    }
    None
}