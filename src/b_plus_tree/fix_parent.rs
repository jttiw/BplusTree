use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::b_plus_tree::{BTree, join};
use crate::block::Block;
use crate::node::Node;

pub(crate) fn fix_parent<T, V>(tree: &mut BTree, mut parent: Node<T, V>, parent_id: usize, neighbor_block_id: usize, neighbor_index_in_parent: usize, passed: Vec<usize>, data: V, right: bool, mut key: T)
    where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
          V: Serialize + DeserializeOwned + Debug + Sized + Clone {
    let parent_content = parent.get_inner_content().expect("error join Node");
    if right {
        parent_content.remove(neighbor_index_in_parent - 1 );
        let parent_content_len = parent_content.len();
        let block2 = Block::to_block(&mut parent);
        tree.bfa.update(parent_id, block2).expect("error btree fix parent");
        if parent_content_len < tree.max / 2 as usize {
            if !passed.is_empty() {
                let parent_parent_id = passed.last().expect("Node join error parent parent").clone();
                let mut parent_parent: Node<T, V> = Node::from_block(&mut tree.bfa.get(parent_parent_id).unwrap());
                let parent_parent_content = parent_parent.get_inner_content().expect("error join Node");
                for i in parent_parent_content {
                    if i.id == parent_id {
                        key = i.comp.clone();
                        break;
                    }
                }
                join::join_node(tree, key, parent_id, passed, data);
            }
            if parent_id == tree.root_id && parent_content_len < 2 {
                let mut new_parent: Node<T, V> = Node::from_block(&mut tree.bfa.get(parent_id).unwrap());
                if new_parent.get_inner_content().expect("error join Node").len() < 2 {
                    tree.set_root(neighbor_block_id);
                }
            }
        }
    }
    else {
        parent_content.remove(neighbor_index_in_parent);
        let parent_content_len = parent_content.len();
        let block2 = Block::to_block(&mut parent);
        tree.bfa.update(parent_id, block2).expect("error btree fix parent");
        if parent_content_len < tree.max / 2 as usize {
            if !passed.is_empty() {
                let parent_parent_id = passed.last().expect("Node join error parent parent").clone();
                let mut parent_parent: Node<T, V> = Node::from_block(&mut tree.bfa.get(parent_parent_id).unwrap());
                let parent_parent_content = parent_parent.get_inner_content().expect("error join Node");
                for i in parent_parent_content {
                    if i.id == parent_id {
                        key = i.comp.clone();
                        break;
                    }
                }
                join::join_node(tree, key, parent_id, passed, data);
            }
            if parent_id == tree.root_id && parent_content_len < 2 {
                let mut new_parent: Node<T, V> = Node::from_block(&mut tree.bfa.get(parent_id).unwrap());
                if new_parent.get_inner_content().expect("error join Node").len() < 2 {
                    tree.set_root(neighbor_block_id);
                }
            }
        }
    }
}