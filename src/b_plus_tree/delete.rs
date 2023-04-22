use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::b_plus_tree::{BTree, join};
use crate::block::Block;
use crate::node::Node;

pub(crate) fn delete_internal<T, V>(tree: &mut BTree, key: T, given: usize, mut passed: Vec<usize>) -> Option<V>
    where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
          V: Serialize + DeserializeOwned + Debug + Sized + Clone {
    let tmp: Node<T, V> = Node::from_block(&mut tree.bfa.get(given).unwrap());
    match tmp {
        Node::LeafNode {mut content, l_ref, r_ref} => {
            for i in 0..content.len() {
                let comp = &content.get(i).unwrap().comp;
                if &key == comp  {
                    let data = content.remove(i).data;
                    let mut new_tmp: Node<T, V> = Node::LeafNode {content: content.clone(), l_ref: l_ref, r_ref: r_ref};
                    let block = Block::to_block(&mut new_tmp);
                    tree.bfa.update(given, block).expect("error btree delete internal");
                    if i == content.len() {
                        for i in 0..passed.len() {
                            let parent_id = passed.get(i).expect("error delete internal").clone();
                            let mut parent: Node<T, V> = Node::from_block(&mut tree.bfa.get(parent_id).unwrap());
                            let newcomp = content.last().expect("error delete internal").comp.clone();
                            let parent_content = parent.get_inner_content().expect("error delete internal");
                            for i in 0..parent_content.len() {
                                let el = parent_content.get(i).unwrap().comp;
                                if el == key {
                                    parent_content[i].comp = newcomp;
                                }
                            }
                            let block2 = Block::to_block(&mut parent);
                            tree.bfa.update(parent_id, block2).expect("error btree delete internal");
                        }
                    }
                    if content.len() < tree.max/2 as usize && !passed.is_empty() {
                        join::join_node(tree, key, given, passed, data.clone());
                    }
                    return Some(data);
                }
            }
        }
        Node::InnerNode {content} => {
            for i in content {
                if i.comp >= key {
                    passed.push(given);
                    return delete_internal(tree, key, i.id, passed);
                }
            }
        }
    }
    return None;
}
