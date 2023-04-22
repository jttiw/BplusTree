use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::b_plus_tree::{BTree, fill};
use crate::b_plus_tree::fix_parent::fix_parent;
use crate::block::Block;
use crate::inner_element::InnerElement;
use crate::leaf_element::LeafElement;
use crate::node::Node;

pub(crate) fn join_leaf<T, V>(tree: &mut BTree, mut underflow: Node<T, V>, mut content: Vec<LeafElement<T, V>>, mut l_ref: Option<usize>, r_ref: Option<usize>, id_for_neighbor: usize, right: bool)
    where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
          V: Serialize + DeserializeOwned + Debug + Sized + Clone {

    let underflow_content = underflow.get_leaf_content().expect("error join Node");
    if right {
        underflow_content.extend(content);
        content = underflow_content.clone();
        l_ref = underflow.get_lref();

        match l_ref {   //set left Neighbors rref to the new, merged Node
            Some(left_ref)=> {
                let mut other_neighbor: Node<T, V> = Node::from_block(&mut tree.bfa.get(left_ref).unwrap());
                other_neighbor.set_rref(Some(id_for_neighbor));
                let block = Block::to_block(&mut other_neighbor);
                tree.bfa.update(left_ref, block).expect("error btree join leaf");
            },
            None=>()
        }

        let mut new_neighbor = Node::LeafNode {content, l_ref, r_ref };
        let block1 = Block::to_block(&mut new_neighbor);
        tree.bfa.update(id_for_neighbor, block1).expect("error btree join leaf");
    }
    else {  //Set right Neighbors lref to new, merged Node
        content.extend(underflow_content.clone());
        underflow.set_leaf_content(content.clone());
        underflow.set_lref(l_ref);
        match l_ref {                       //TODO THis cant be right -> should check r_ref and set right neighbors l_ref to new merged Node -> not the l_ref in the Signature? -> Test a join Node with left Neighbor
            Some(left_ref)=> {
                let mut other_neighbor: Node<T, V> = Node::from_block(&mut tree.bfa.get(left_ref).unwrap());
                other_neighbor.set_rref(Some(id_for_neighbor));
                let block = Block::to_block(&mut other_neighbor);
                tree.bfa.update(left_ref, block).expect("error btree join leaf");
            },
            None=>()
        }
        let block1 = Block::to_block(&mut underflow);
        tree.bfa.update(id_for_neighbor, block1).expect("error btree join leaf");
    }
}

pub(crate) fn join_inner<T, V>(tree: &mut BTree, mut underflow: Node<T, V>, mut content: Vec<InnerElement<T>>, id_for_neighbor: usize, right: bool)
    where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
          V: Serialize + DeserializeOwned + Debug + Sized + Clone {
    let underflow_content = underflow.get_inner_content().expect("error join Node");

    if right {
        underflow_content.extend(content);
        content = underflow_content.clone();
        let mut new_neighbor: Node<T, V> = Node::InnerNode {content };
        let block1 = Block::to_block(&mut new_neighbor);
        tree.bfa.update(id_for_neighbor, block1).expect("error btree join inner");
    }
    else {
        content.extend(underflow_content.iter());
        underflow.set_inner_content(content.clone());
        let block1 = Block::to_block(&mut underflow);
        tree.bfa.update(id_for_neighbor, block1).expect("error btree join inner");
    }
}

pub(crate) fn join_node<T, V>(tree: &mut BTree, key: T, given: usize, mut passed: Vec<usize>, data: V)
    where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
          V: Serialize + DeserializeOwned + Debug + Sized + Clone {
    let parent_id = passed.pop().expect("join Node error");
    let mut underflow: Node<T, V> = Node::from_block(&mut tree.bfa.get(given).unwrap());
    let mut parent: Node<T, V> = Node::from_block(&mut tree.bfa.get(parent_id).unwrap());
    let mut right = true;
    let mut neighbor_index_in_parent: usize = 0;
    let parent_content = parent.get_inner_content().expect("error join Node");
    for i in 0..parent_content.len() {
        if given == parent_content.get(i).expect("join Node error").id {
            if i == parent_content.len() - 1 {
                neighbor_index_in_parent = i - 1;
                right = false;
                break;
            }
            neighbor_index_in_parent = i + 1;
            break;
        }
    }
    let neighbor_block_id = parent_content.get_mut(neighbor_index_in_parent).expect("error join Node").id;
    let neighbor: Node<T, V> = Node::from_block(&mut tree.bfa.get(neighbor_block_id).unwrap());
    match neighbor {
        Node::LeafNode {content, l_ref, r_ref} => {
            //neighbor ist zur hälfte gefüllt
            if content.len()  <= tree.max/2 as usize {
                //rechter neighbor ist zur hälfte gefüllt
                if right {
                    join_leaf(tree, underflow, content, l_ref, r_ref, neighbor_block_id, right);
                    fix_parent(tree, parent, parent_id, neighbor_block_id, neighbor_index_in_parent, passed, data, right, key);
                }
                //linker neighbor ist zur hälfte gefüllt
                else {
                    join_leaf(tree, underflow, content, l_ref, r_ref, given, right);
                    fix_parent(tree, parent, parent_id, given, neighbor_index_in_parent, passed, data, right, key);
                }
            }
            else {
                let underflow_content = underflow.get_leaf_content().expect("error join Node");
                //underflow und neighbor können verschmolzen werden
                if underflow_content.len() + content.len() <= tree.max {
                    //neighbor ist rechts
                    if right {
                        join_leaf(tree, underflow, content, l_ref, r_ref, neighbor_block_id, right);
                        fix_parent(tree, parent, parent_id, neighbor_block_id, neighbor_index_in_parent, passed, data, right, key);
                    }
                    //neighbor ist links
                    else {
                        join_leaf(tree, underflow, content, l_ref, r_ref, given, right);
                        fix_parent(tree, parent, parent_id, given, neighbor_index_in_parent, passed, data, right, key);
                    }
                }
                //underflow und neighbor können nicht verschmolzen werden, nehme dann wenn möglich den rechten nachbar und füllt underflow damit auf
                else {
                    //neighbor ist rechts
                    if right{
                        fill::fill_leaf(tree, underflow, content, l_ref, r_ref, parent, given, neighbor_block_id, parent_id, neighbor_index_in_parent, right);
                    }
                    //neighbor ist links
                    else {
                        fill::fill_leaf(tree, underflow, content, l_ref, r_ref, parent, given, neighbor_block_id, parent_id, neighbor_index_in_parent, right);
                    }
                }
            }
        }
        Node::InnerNode {content} => {
            //neighbor ist zur hälfte gefüllt
            if content.len()  <= tree.max/2 as usize {
                //rechter neighbor ist zur hälfte gefüllt
                if right {
                    join_inner(tree, underflow, content, neighbor_block_id, right);
                    fix_parent(tree, parent, parent_id, neighbor_block_id, neighbor_index_in_parent, passed, data, right, key);
                }
                //linker neighbor ist zur hälfte gefüllt
                else {
                    join_inner(tree, underflow, content, given, right);
                    fix_parent(tree, parent, parent_id, given, neighbor_index_in_parent, passed, data, right, key);
                }
            }
            else {
                let underflow_content = underflow.get_inner_content().expect("error join Node");
                //underflow und neighbor können verschmolzen werden
                if underflow_content.len() + content.len() <= tree.max {
                    //neighbor ist rechts
                    if right {
                        join_inner(tree, underflow, content, neighbor_block_id, right);
                        fix_parent(tree, parent, parent_id, neighbor_block_id, neighbor_index_in_parent, passed, data, right, key);
                    }
                    //neighbor ist links
                    else {
                        join_inner(tree, underflow, content, given, right);
                        fix_parent(tree, parent, parent_id, given, neighbor_index_in_parent, passed, data, right, key);
                    }
                }
                //underflow und neighbor können nicht verschmolzen werden, nehme dann wenn möglich den rechten nachbar und füllt underflow damit auf
                else {
                    //neighbor ist rechts
                    if right {
                        fill::fill_inner(tree, underflow, content, parent, given, neighbor_block_id, parent_id, neighbor_index_in_parent, right);
                    }
                    //neighbor ist links
                    else {
                        fill::fill_inner(tree, underflow, content, parent, given, neighbor_block_id, parent_id, neighbor_index_in_parent, right);
                    }
                }
            }
        }
    }
}