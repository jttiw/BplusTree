use crate::student::Student;
use crate::bplus_tree::BTree;

#[test]
pub fn test_btree_search_interval() {
    let mut btree = BTree::new(500,"tests","btree_test_search_internal");
    btree.set_up();
    for i in 0..100 {
        let mut student = Student::new("Witt", "Jonathan", i as usize);
        btree.insert(i ,  student);
    }
    let students: Vec<Student> = btree.search_interval(0 , 99 ).unwrap();
    for i in 0..100 {
        assert_eq!(students[i].matrikelnr, i);
    }
    btree.close();
}

#[test]
pub fn test_btree_search() {
    let mut btree = BTree::new(500,"tests","btree_test_search");
    btree.set_up();
    for i in 0..100 {
        let mut student = Student::new("Witt", "Jonathan", i as usize);
        btree.insert(i ,  student);
    }
    for i in 0..100 {
        let student: Student = btree.search(i ).unwrap();
        assert_eq!(student.matrikelnr, i as usize);
    }
    btree.close();
}

#[test]
pub fn test_btree_search_none() {
    let mut btree = BTree::new(500,"tests","test_btree_search_none");
    btree.set_up();
    for i in 0..100 {
            let mut student = Student::new("Witt", "Jonathan", i as usize);
            btree.insert(i ,  student);
        }
    let students:Option<Vec<Student>> = btree.search_interval(5 , 4 );
    assert_eq!(students.is_none(),true);
     btree.close();
}

#[test]
pub fn test_btree_insert_front_of_node_max_3() {
    let mut btree = BTree::new(120,"tests","test_btree_insert_front_of_node_max_3");
    btree.set_up();
    btree.insert(2, 0 as usize);
    btree.insert(4, 1 as usize);
    btree.insert(6, 2 as usize);
    btree.insert(1, 3 as usize);
    btree.print(&2, 5 as usize);
    let v: Vec<i32> = vec![2,1,2,6,4,6];
    let res: Vec<i32> = Vec::new();
    assert_eq!(btree.traverse(btree.root_id,6 as usize,res).unwrap(),v);
    btree.close();
}

#[test]
pub fn test_btree_insert_second_of_node_max_3() {
    let mut btree = BTree::new(120,"tests","test_btree_insert_second_of_node_max_3");
    btree.set_up();
    btree.insert(2, 0 as usize);
    btree.insert(4, 1 as usize);
    btree.insert(6, 2 as usize);
    btree.insert(3, 3 as usize);
    btree.print(&2, 5 as usize);
    let v: Vec<i32> = vec![3,2,3,6,4,6];
    let res: Vec<i32> = Vec::new();
    assert_eq!(btree.traverse(btree.root_id,6 as usize,res).unwrap(),v);
    btree.close();
}

#[test]
pub fn test_btree_insert_third_of_node_max_3() {
    let mut btree = BTree::new(120,"tests","test_btree_insert_third_of_node_max_3");
    btree.set_up();
    btree.insert(2, 0 as usize);
    btree.insert(4, 1 as usize);
    btree.insert(6, 2 as usize);
    btree.insert(5, 3 as usize);
    btree.print(&2, 5 as usize);
    let v: Vec<i32> = vec![4,2,4,6,5,6];
    let res: Vec<i32> = Vec::new();
    assert_eq!(btree.traverse(btree.root_id,6 as usize,res).unwrap(),v);
    btree.close();
}

#[test]
pub fn test_btree_insert_fourth_of_node_max_3() {
    let mut btree = BTree::new(120,"tests","test_btree_insert_fourth_of_node_max_3");
    btree.set_up();
    btree.insert(2, 0 as usize);
    btree.insert(4, 1 as usize);
    btree.insert(6, 2 as usize);
    btree.insert(7, 3 as usize);
    btree.print(&2, 5 as usize);
    let v: Vec<i32> = vec![4,2,4,7,6,7];
    let res: Vec<i32> = Vec::new();
    assert_eq!(btree.traverse(btree.root_id,6 as usize,res).unwrap(),v);
    btree.close();
}

//test join-delete
#[test]
pub fn test_btree_delete_max_4() {
    let mut btree = BTree::new(160,"tests","test_btree_delete_max_4");
    btree.set_up();
    btree.insert(3, 0 as usize);
    btree.insert(4, 1 as usize);
    btree.insert(6, 2 as usize);
    btree.insert(2, 3 as usize);
    btree.insert(1, 4 as usize);
    let o:Option<usize> = btree.delete(2);
    let v: Vec<i32> = vec![1,3,4,6];
    let res: Vec<i32> = Vec::new();
    assert_eq!(btree.traverse(btree.root_id,6 as usize,res).unwrap(),v);
    btree.close();
}

//test regular delete
#[test]
pub fn test_btree_delete2_max_4() {
    let mut btree = BTree::new(160,"tests","test_btree_delete2_max_4");
    btree.set_up();
    btree.insert(3, 0 as usize);
    btree.insert(4, 1 as usize);
    btree.insert(6, 2 as usize);
    btree.insert(2, 3 as usize);
    btree.insert(1, 4 as usize);
    let o:Option<usize> = btree.delete(6);
    let v: Vec<i32> = vec![2,1,2,4,3,4];
    let res: Vec<i32> = Vec::new();
    assert_eq!(btree.traverse(btree.root_id,6 as usize,res).unwrap(),v);
    btree.close();
}
//delete join mit rechtem Nachbar unterlauf
#[test]
pub fn test_btree_delete_join_right_max_4() {
    let mut btree = BTree::new(160,"tests","test_btree_delete_join_right_max_4");
    btree.set_up();
    btree.insert(4, 0 as usize);
    btree.insert(3, 1 as usize);
    btree.insert(6, 2 as usize);
    btree.insert(2, 3 as usize);
    btree.insert(1, 4 as usize);
    btree.insert(8, 5 as usize);
    btree.insert(10, 6 as usize);
    let o:Option<usize> = btree.delete(3);
    let v: Vec<i32> = vec![2,1,2,10,4,6,8,10];
    let res: Vec<i32> = Vec::new();
    assert_eq!(btree.traverse(btree.root_id,6 as usize,res).unwrap(),v);
    btree.close();
}

//delete rechter Nachbar füllt underflow auf
#[test]
pub fn test_btree_delete_fill_with_right_node_max_4() {
    let mut btree = BTree::new(160,"tests","test_btree_delete_fill_with_right_node_max_4");
    btree.set_up();
    btree.insert(4, 0 as usize);
    btree.insert(3, 1 as usize);
    btree.insert(2, 3 as usize);
    btree.insert(1, 4 as usize);
    btree.insert(10, 6 as usize);
    btree.insert(11, 7 as usize);
    btree.insert(12, 7 as usize);
    btree.insert(14, 8 as usize);

    let o:Option<usize> = btree.delete(3);
    let v: Vec<i32> = vec![2,1,2,10,4,10,14,11,12,14];
    let res: Vec<i32> = Vec::new();
    assert_eq!(btree.traverse(btree.root_id,6 as usize,res).unwrap(),v);
    btree.close();
}

//delete join mit linkem Nachbar unterlauf
#[test]
pub fn test_btree_delete_join_left_max_4() {
    let mut btree = BTree::new(160,"tests","test_btree_delete_join_left_max_4");
    btree.set_up();
    btree.insert(2, 3 as usize);
    btree.insert(1, 4 as usize);
    btree.insert(6, 7 as usize);
    btree.insert(12, 7 as usize);
    btree.insert(14, 8 as usize);
    btree.insert(5, 6 as usize);
    btree.insert(4, 0 as usize);
    btree.insert(3, 1 as usize);

    let o:Option<usize> = btree.delete(6);
    let o:Option<usize> = btree.delete(12);

    let v: Vec<i32> = vec![2, 1, 2, 14, 3, 4, 5, 14];
    let res: Vec<i32> = Vec::new();
    assert_eq!(btree.traverse(btree.root_id,14 as usize,res).unwrap(),v);

    btree.close();
}

//delete linker Nachbar füllt underflow auf
#[test]
pub fn test_btree_delete_fill_with_left_node_max_4() {
    let mut btree = BTree::new(160,"tests","test_btree_delete_fill_with_left_node_max_4");
    btree.set_up();
    btree.insert(12, 7 as usize);
    btree.insert(9, 19 as usize);
    btree.insert(11, 21 as usize);
    btree.insert(8, 18 as usize);
    btree.insert(7, 17 as usize);
    btree.insert(1, 4 as usize);
    btree.insert(2, 3 as usize);
    btree.insert(3, 1 as usize);
    btree.insert(4, 0 as usize);
    btree.insert(5, 6 as usize);
    btree.insert(6, 7 as usize);
    btree.insert(10, 20 as usize);

    let o:Option<usize> = btree.delete(9);
    let o:Option<usize> = btree.delete(10);
    let o:Option<usize> = btree.delete(11);

    let v: Vec<i32> = vec![2, 1, 2, 4, 3, 4, 7, 5, 6, 7, 12, 8, 12];
    let res: Vec<i32> = Vec::new();
    assert_eq!(btree.traverse(btree.root_id,18 as usize,res).unwrap(),v);

    btree.close();
}

#[test]
pub fn test_btree_delete_none_exist() {
        let mut btree = BTree::new(500,"tests","test_btree_delete_none_exist");
        btree.set_up();
        for i in 0..100 {
                let mut student = Student::new("Witt", "Jonathan", i as usize);
                btree.insert(i ,  student);
            }

        let students: Option<Vec<Student>> = btree.delete(999);
        assert_eq!(students.is_none(),true);
        btree.close();
    }

#[test]
pub fn test_btree_delete_usize_as_comp() {
    let mut btree = BTree::new(400,"tests","test_btree_delete_usize_as_comp");
    btree.set_up();
    for i in 0..50 {
        let mut student = Student::new("Witt", "Jonathan", i as usize);
        btree.insert(student.matrikelnr,  student);
    }
    for i in (0..50).rev() {
        if i%3 == 0 {
            let data: Option<Student> = btree.delete(i as usize);
            match data {
                None => {
                    println!("ERROR!!!");
                    break;
                }
                _  => {

                }
            }

        }
    }
    let students: Vec<Student> = btree.search_interval(0 as usize, 50 as usize).unwrap();
    for i in 0..students.len() {
        assert_ne!(students[i].matrikelnr % 3, 0);
    }
    btree.close();
}
