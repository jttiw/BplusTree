use fopra_block_file_access::bplus_tree::BTree;

fn main() {


    let mut btree = BTree::new(120,"tests","btree_test_insert_front_of_node_max_3");
    btree.set_up();
    btree.insert(2, 0 as usize).unwrap();
    btree.insert(4, 1 as usize).unwrap();
    btree.insert(6, 2 as usize).unwrap();
    btree.insert(1, 3 as usize).unwrap();
    btree.print(&2, 5 as usize);
    let v: Vec<i32> = vec![2,1,2,6,4,6];
    let res: Vec<i32> = Vec::new();
    assert_eq!(btree.traverse(btree.root_id,6 as usize,res).unwrap(),v);
    btree.close();

}
