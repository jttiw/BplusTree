use std::fmt::{Debug, Error};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::b_plus_tree::BTree;
use crate::block::Block;
use crate::inner_element::InnerElement;
use crate::leaf_element::LeafElement;
use crate::node::Node;
use crate::b_plus_tree::split;

//insert el in Tree
//if id in tree -> None else -> Error Message
//first call : given = Node::ROOT_ID
//passed : keep track of all traversed Nodes to operate on parents if needed
//update : all changed Nodes -> Blocks have to be updated
pub(crate) fn b_tree_insert<T, V> (tree: &mut BTree, element: LeafElement<T, V>, given: usize, mut passed: Vec<usize>) -> Result<(), Error>
    where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
          V: Serialize + DeserializeOwned + Debug + Sized + Clone {

    let tmp: Node<T, V> = Node::from_block(&mut tree.bfa.get(given).unwrap());
    match tmp {
        Node::LeafNode {mut content, l_ref, r_ref} => {
            for i in 0..content.len() {
                if &element.comp < &content.get(i).unwrap().comp {
                    if &content.len() < &tree.max {
                        content.insert(i, element);
                        let mut new_tmp = Node::LeafNode {content, l_ref, r_ref };
                        let tmp_block = Block::to_block( &mut new_tmp);
                        tree.bfa.update(given, tmp_block).expect("error b_tree_insert");
                        return Ok(());
                    }
                    else { // falls Node voll
                        split::split_leaf(tree, element.clone(), passed, element.data.clone());
                        //tree.split_Node(Some(element.clone()), None, &mut passed, element.data.clone());
                        return Ok(());
                    }
                }
                else if element.comp == content.get(i).unwrap().comp {
                    return Err(Error);
                }
            }
            // element größer als alle im Knoten
            if &content.len() < &tree.max {
                content.push( element);
                let mut new_tmp = Node::LeafNode {content, l_ref, r_ref };
                let tmp_block = Block::to_block(&mut new_tmp);
                tree.bfa.update(given, tmp_block).expect("error b_tree_insert");
                Ok(())
            }
            else {
                split::split_leaf(tree, element.clone(), passed, element.data.clone());
                //tree.split_Node(Some(element.clone()), None, &mut passed, element.data);
                Ok(())
            }
        }
        Node::InnerNode {mut content} => {
            for i in &content {
                if element.comp <= i.comp {
                    passed.push(i.id);
                    return b_tree_insert(tree, element,i.id ,passed);
                }
            }
            //el comp größer als alle im Knoten -> letzten Index = el.comp
            let mut tmp_el: InnerElement<T> = content.pop().expect("error b tree insert");
            tmp_el.comp = element.comp.clone();
            passed.push(tmp_el.id);
            content.push(tmp_el);
            let mut new_tmp: Node<T, V> = Node::InnerNode{content };
            let tmp_block = Block::to_block(&mut new_tmp);
            tree.bfa.update(given, tmp_block).expect("error btree b_tree_insert");
            return b_tree_insert(tree, element,tmp_el.id, passed);
        }
    }
}