use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::b_plus_tree::BTree;
use crate::b_plus_tree::traverse::traverse_internal;
use crate::block::Block;
use crate::inner_element::InnerElement;
use crate::leaf_element::LeafElement;
use crate::node::Node;

pub(crate) fn fill_leaf<T, V>(tree: &mut BTree, mut underflow: Node<T, V>, mut content: Vec<LeafElement<T, V>>, l_ref: Option<usize>, r_ref: Option<usize>, mut parent: Node<T, V>, given: usize, neighbor_block_id: usize, parent_id: usize, neighbor_index_in_parent: usize, right: bool)
    where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
          V: Serialize + DeserializeOwned + Debug + Sized + Clone {
    let parent_content = parent.get_inner_content().expect("error fill leaf");

    if right {
        let underflow_content = underflow.get_leaf_content().expect("error fill leaf");
        while underflow_content.len() < tree.max / 2 as usize {
            let a = content.first().expect("error fill leaf").clone();
            content.remove(0);
            underflow_content.push(a);
        }
        let comp = underflow_content.last().expect("error join Node").comp.clone();
        parent_content.get_mut(neighbor_index_in_parent-1).unwrap().comp = comp;

    }
    else {
        let underflow_content = underflow.get_leaf_content().unwrap();
        while underflow_content.len() < tree.max / 2 as usize {
            let a = content.pop().unwrap();
            underflow_content.insert(0, a);
        }
        let comp = content.last().unwrap().comp.clone();
        parent_content.get_mut(neighbor_index_in_parent).unwrap().comp = comp;
    }

    let mut new_neighbor = Node::LeafNode {content, l_ref, r_ref };
    let block1 = Block::to_block(&mut underflow);
    let block2 = Block::to_block(&mut new_neighbor);
    let block3 = Block::to_block(&mut parent);
    tree.bfa.update(given, block1).expect("error fill leaf");
    tree.bfa.update(neighbor_block_id, block2).expect("error fill leaf");
    tree.bfa.update(parent_id, block3).expect("error fill leaf");
}

pub(crate) fn fill_inner<T, V>(tree: &mut BTree, mut underflow: Node<T, V>, mut content: Vec<InnerElement<T>>, mut parent: Node<T, V>, given: usize, neighbor_block_id: usize, parent_id: usize, neighbor_index_in_parent: usize, right: bool)
    where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
          V: Serialize + DeserializeOwned + Debug + Sized + Clone {

    let parent_content = parent.get_inner_content().expect("error fill inner");
    let underflow_content = underflow.get_inner_content().expect("error fill inner");
    if right {
        while underflow_content.len() < tree.max / 2 as usize {
            let a = content.first().expect("error fill inner").clone();
            content.remove(0);
            underflow_content.push(a);
        }
        let comp = underflow_content.last().expect("error fill inner").comp.clone();
        parent_content.get_mut(neighbor_index_in_parent-1).expect("error join Node").comp = comp;
    }
    else {
        while underflow_content.len() < tree.max / 2 as usize {
            let a = content.pop().expect("error fill inner");
            underflow_content.insert(0, a);
        }
        let comp = content.last().expect("error fill inner").comp.clone();
        parent_content.get_mut(neighbor_index_in_parent).expect("error fill inner").comp = comp;
    }

    let mut new_neighbor: Node<T, V> = Node::InnerNode {content: content};
    let block1 = Block::to_block(&mut underflow);
    let block2 = Block::to_block(&mut new_neighbor);
    let block3 = Block::to_block(&mut parent);
    tree.bfa.update(given, block1).expect("error fill inner");
    tree.bfa.update(neighbor_block_id, block2).expect("error fill inner");
    tree.bfa.update(parent_id, block3).expect("error fill inner");
}