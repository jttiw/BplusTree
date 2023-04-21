use fopra_block_file_access::b_plus_tree::BTree;

fn main() {

    let mut btree = BTree::new(120,"files","A_main_test");
    btree.set_up();
    /*
    btree.insert(2, 0 as usize).unwrap();
    btree.insert(4, 1 as usize).unwrap();
    btree.insert(6, 2 as usize).unwrap();
    btree.insert(1, 3 as usize).unwrap();
    */
    btree.insert(3, 3 as usize).unwrap();


    btree.print::<i32, usize>(None);

    let v: Vec<i32> = vec![2,1,2,6,4,6];

    let res: Vec<i32> = btree.traverse::<i32, usize>().unwrap();
    //btree.print::<i32,usize>(None);
    //println!("{}", v.eq(&res));
    println!("{:?}", &res);
    let file_content = btree.bfa.blocks();
    println!("#blocks {:?}", file_content.len());
    btree.close();

}
