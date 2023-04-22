use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::b_plus_tree::BTree;
use crate::block::Block;
use crate::inner_element::InnerElement;
use crate::leaf_element::LeafElement;
use crate::node::Node;

pub(crate) fn split_leaf <T, V> (tree: &mut BTree, leaf_element: LeafElement<T, V>, mut passed: Vec<usize>, data: V)
    where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
          V: Serialize + DeserializeOwned + Debug + Sized + Clone {

    let current_id = passed.pop().unwrap();
    let tmp: Node<T, V> = Node::from_block(&mut tree.bfa.get(current_id).unwrap());
    let mut b = false; //Element was inserted: if false element.comp bigger than all comps in Node
    match tmp {
        Node::LeafNode {mut content, l_ref, mut r_ref} => {
            let comp = leaf_element.clone().comp;
            for i in 0..content.len() {
                if comp < content.get(i).unwrap().comp {
                    content.insert(i, leaf_element.clone());
                    b = true;
                    break;
                }
            }
            if !b {
                content.push( leaf_element);
            }

            let second_half = content.split_off((content.len() / 2) as usize);
            //new Node
            let parent_comp = second_half.last().expect("error split leaf").comp;
            //update second half
            let mut tmp_node2 = Node::LeafNode{content: second_half, l_ref: Some(current_id), r_ref: r_ref};   //-> right Node; tmp -> left Node
            let node2_id = tree.bfa.reserve();
            let block2 = Block::to_block(&mut tmp_node2);
            tree.bfa.update(node2_id, block2).expect("error btree split leaf");

            //update right neighbor if exists
            match r_ref {
                Some(id_of_right_neighbor)=>{
                    let mut right_neighbor: Node<T, V> = Node::from_block(&mut tree.bfa.get(id_of_right_neighbor ).unwrap());
                    right_neighbor.set_lref(Some(node2_id));
                    let block = Block::to_block(&mut right_neighbor);
                    tree.bfa.update(id_of_right_neighbor , block).expect("error btree split leaf");
                }
                None=>()

            }
            //update first half
            r_ref = Some(node2_id);
            let mut new_tmp = Node::LeafNode {content: content.clone(), l_ref: l_ref, r_ref: r_ref};
            let block1 = Block::to_block(&mut new_tmp);
            tree.bfa.update(current_id, block1).expect("error btree split leaf");

            //update parent
            let new_el_for_parent = InnerElement::new(parent_comp, node2_id);
            if passed.len() != 0 {
                let id = *passed.last().unwrap();
                tree.adjust_ref(content.last().unwrap().comp, id, data.clone());
                //Rekursion
                if !(tree.insert_into_node(new_el_for_parent, *passed.last().unwrap(), data.clone())) {
                    split_inner(tree, new_el_for_parent, passed, data.clone());
                }
            }
            else { //make new root-Node
                let el2_for_root = InnerElement::new(tmp_node2.get_leaf_content().unwrap().last().unwrap().comp, node2_id);
                let el1_for_root = InnerElement::new(content.last().unwrap().comp, current_id);
                tree.new_root(el1_for_root, el2_for_root, data.clone(), current_id);
            }
        }
        _ => {
            println!("Node {}, was not a LeafNode -> should not happen as LeafNode refs only point to other LeafNodes", current_id); //TODO passed contains Inner -> tests split
        }
    }
}

pub(crate) fn split_inner <T, V> (tree: &mut BTree, inner_element: InnerElement<T>, mut passed: Vec<usize>, data: V)
    where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
          V: Serialize + DeserializeOwned + Debug + Sized + Clone {
    let current_id = passed.pop().unwrap();
    let tmp: Node<T, V> = Node::from_block(&mut tree.bfa.get(current_id).unwrap());
    let mut b = false;
    match tmp {
        Node::InnerNode {mut content} => {
            for i in 0..content.len() {
                if &inner_element.comp < &content.get(i).unwrap().comp {
                    content.insert(i, inner_element);
                    b = true;
                    break;
                }
            }
            if !b {
                content.push( inner_element);
            }

            let second_half = content.split_off((content.len() / 2) as usize); //check to use mult instead of div for efficiency)
            //neue Node
            let mut tmp_node2: Node<T, V> = Node::InnerNode{content: second_half};
            let node2_id = tree.bfa.reserve();
            let new_el_for_parent = InnerElement::new(tmp_node2.get_inner_content().unwrap().last().unwrap().comp, node2_id);
            let block2 = Block::to_block(&mut tmp_node2);
            tree.bfa.update(node2_id, block2).expect("error btree split inner");
            let mut new_tmp: Node<T, V> = Node::InnerNode{content: content.clone()};
            let block1 = Block::to_block(&mut new_tmp);
            tree.bfa.update(current_id, block1).expect("error btree split inner");

            if passed.len() != 0 {
                let id = *passed.last().unwrap();
                tree.adjust_ref(content.last().unwrap().comp, id, data.clone());
                //Rekursion
                if !(tree.insert_into_node(new_el_for_parent, *passed.last().unwrap(), data.clone())) {
                    split_inner(tree, new_el_for_parent, passed, data.clone());
                }
            }
            else { //make new root-Node
                let el2_for_root = InnerElement::new(tmp_node2.get_inner_content().unwrap().last().unwrap().comp, node2_id);
                let el1_for_root = InnerElement::new(content.last().unwrap().comp, current_id);
                tree.new_root(el1_for_root, el2_for_root, data.clone(), current_id);
            }
        }
        _ => {
            println!("Something went wrong splitting Node {}", current_id);
        }
    }
}