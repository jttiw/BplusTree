use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::b_plus_tree::BTree;
use crate::node::Node;

pub(crate) fn print_tree_structure<T, V>(tree: &mut BTree, given: usize)
    where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
          V: Serialize + DeserializeOwned + Debug + Sized + Clone {

    println!();
    let tmp: Node<T, V> = Node::from_block(&mut tree.bfa.get(given).unwrap());
    print!("id: {:?} - ", given);
    match tmp {
        Node::LeafNode {content, l_ref, r_ref} => {
            print!("leaf [");
            for i in 0..content.len() {
                print!("{:?}", content.get(i).unwrap().comp);
                if i < content.len() - 1 {
                    print!(" | ");
                }
            }
            print!("]");
            print!(" - lref :{:?} ",l_ref);
            print!("- rref :{:?}", r_ref);
        }
        Node::InnerNode {content} => {
            print!("[");
            for i in 0..content.len() {
                print!("{:?}", content.get(i).unwrap().comp);
                if i < content.len() - 1 {
                    print!(" | ");
                }
            }
            print!("]");
            for i in &content {
                print_tree_structure::<T, V>(tree, i.id);
            }
        }
    }
}