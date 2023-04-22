use crate::node::Node;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use crate::b_plus_tree::BTree;

//search x
//first call -> given = BTree.root_id
// prints out passed Nodes + el for debug purposes
pub(crate) fn search_internal<T, V> (tree: &mut BTree, x: T, given: usize) -> Option<V>
    where T: DeserializeOwned + Debug + Ord,
          V: DeserializeOwned + Debug + Sized {
    let tmp: Node<T, V> = Node::from_block(&mut tree.bfa.get(given).expect("error search internal"));
    match tmp {
        Node::LeafNode {content, l_ref: _, r_ref: _} => {
            for i in content {
                if x == i.comp {
                    return Some(i.data);
                }
            }
        }
        Node::InnerNode {content} => {
            for i in content {
                if i.comp >= x {
                    return search_internal(tree, x, i.id);
                }
            }
        }
    }
    None
}