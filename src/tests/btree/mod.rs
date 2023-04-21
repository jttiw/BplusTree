#[cfg(test)]
mod btree_tests{
    use crate::student::Student;
    use crate::b_plus_tree::BTree;


    // BETTER Use setup scopes
    //TODO clean up function - remove all tests files via Rust -> Dir Walker
    //Also have to change BFA::new to take dirs in all OS formats -> Currently Linux format {dir}/{filename}
    /*
    use std::fs::remove_file;
    use std::process::{Command, Output};

    #[tests]
    pub fn test_cleanup() {
        let dir = std::env::current_dir().expect("current dir fail");
        dbg!(dir.to_str().unwrap().to_owned() + "/files/");
        //let  = Command::new("sh").arg("cd").arg(dir.to_str().unwrap().to_owned()).arg("&&").arg("ls").output().unwrap(); //.arg("&&").arg("rm").arg("*btree*").output().unwrap();
        assert_eq!(.stdout, vec![]);
    }*/

    //WARNING: These Tests do not work a second time, as the BFA is persistent ie. files were created and content was inserted -> inserting same elements returns an error
    //Use cleanup function to et
    #[test]
    pub fn test_btree_search_interval() {
        let mut btree = BTree::new(500,"test_files","btree_test_search_internal");
        btree.set_up();
        for i in 0..100 {
            let student = Student::new("Witt", "Jonathan", i as usize);
            btree.insert(i ,  student).unwrap();
        }
        let students: Vec<Student> = btree.search_interval(0 , 99 ).unwrap();
        for i in 0..100 {
            assert_eq!(students[i].matrikelnr, i);
        }
        btree.close();
    }

    #[test]
    pub fn test_btree_search() {
        let mut btree = BTree::new(500,"test_files","btree_test_search");
        btree.set_up();
        for i in 0..100 {
            let student = Student::new("Witt", "Jonathan", i as usize);
            btree.insert(i ,  student).unwrap();
        }
        for i in 0..100 {
            let student: Student = btree.search(i ).unwrap();
            assert_eq!(student.matrikelnr, i as usize);
        }
        btree.close();
    }

    #[test]
    pub fn test_btree_search_none() {
        let mut btree = BTree::new(500,"test_files","btree_test_search_none");
        btree.set_up();
        for i in 0..100 {
            let student = Student::new("Witt", "Jonathan", i as usize);
            btree.insert(i ,  student).unwrap();
        }
        let students:Option<Vec<Student>> = btree.search_interval(5 , 4 );
        assert_eq!(students.is_none(),true);
        btree.close();
    }

    #[test]
    pub fn test_btree_insert_front_of_node_max_3() {
        let mut btree = BTree::new(120,"test_files","btree_test_insert_front_of_node_max_3");
        btree.set_up();
        btree.insert(2, 0 as usize).unwrap();
        btree.insert(4, 1 as usize).unwrap();
        btree.insert(6, 2 as usize).unwrap();
        btree.insert(1, 3 as usize).unwrap();
        btree.print::<i32, usize>(Some(2));
        let v: Vec<i32> = vec![2,1,2,6,4,6];
        assert_eq!(btree.traverse::<i32, usize>().unwrap(), v);
        btree.close();
    }

    #[test]
    pub fn test_btree_insert_second_of_node_max_3() {
        let mut btree = BTree::new(120,"test_files","btree_test_insert_second_of_node_max_3");
        btree.set_up();
        btree.insert(2, 0 as usize).unwrap();
        btree.insert(4, 1 as usize).unwrap();
        btree.insert(6, 2 as usize).unwrap();
        btree.insert(3, 3 as usize).unwrap();
        btree.print::<i32, usize>(Some(2));
        let v: Vec<i32> = vec![3,2,3,6,4,6];
        assert_eq!(btree.traverse::<i32, usize>().unwrap(), v);
        btree.close();
    }

    #[test]
    pub fn test_btree_insert_third_of_node_max_3() {
        let mut btree = BTree::new(120,"test_files","btree_test_insert_third_of_node_max_3");
        btree.set_up();
        btree.insert(2, 0 as usize).unwrap();
        btree.insert(4, 1 as usize).unwrap();
        btree.insert(6, 2 as usize).unwrap();
        btree.insert(5, 3 as usize).unwrap();
        btree.print::<i32, usize>(Some(2));
        let v: Vec<i32> = vec![4,2,4,6,5,6];
        assert_eq!(btree.traverse::<i32, usize>().unwrap(), v);
        btree.close();
    }

    #[test]
    pub fn test_btree_insert_fourth_of_node_max_3() {
        let mut btree = BTree::new(120,"test_files","btree_test_insert_fourth_of_node_max_3");
        btree.set_up();
        btree.insert(2, 0 as usize).unwrap();
        btree.insert(4, 1 as usize).unwrap();
        btree.insert(6, 2 as usize).unwrap();
        btree.insert(7, 3 as usize).unwrap();
        btree.print::<i32, usize>(Some(2));
        let v: Vec<i32> = vec![4,2,4,7,6,7];
        assert_eq!(btree.traverse::<i32, usize>().unwrap(), v);
        btree.close();
    }

    //tests join-delete
    #[test]
    pub fn test_btree_delete_max_4() {
        let mut btree = BTree::new(160,"test_files","btree_test_delete_max_4");
        btree.set_up();
        btree.insert(3, 0 as usize).unwrap();
        btree.insert(4, 1 as usize).unwrap();
        btree.insert(6, 2 as usize).unwrap();
        btree.insert(2, 3 as usize).unwrap();
        btree.insert(1, 4 as usize).unwrap();
        let _o:Option<usize> = btree.delete(2);
        let v: Vec<i32> = vec![1,3,4,6];
        assert_eq!(btree.traverse::<i32, usize>().unwrap(), v);
        btree.close();
    }

    //tests regular delete
    #[test]
    pub fn test_btree_delete2_max_4() {
        let mut btree = BTree::new(160,"test_files","btree_test_delete2_max_4");
        btree.set_up();
        btree.insert(3, 0 as usize).unwrap();
        btree.insert(4, 1 as usize).unwrap();
        btree.insert(6, 2 as usize).unwrap();
        btree.insert(2, 3 as usize).unwrap();
        btree.insert(1, 4 as usize).unwrap();
        let _o:Option<usize> = btree.delete(6);
        let v: Vec<i32> = vec![2,1,2,4,3,4];
        assert_eq!(btree.traverse::<i32, usize>().unwrap(), v);
        btree.close();
    }
    //delete join mit rechtem Nachbar unterlauf
    #[test]
    pub fn test_btree_delete_join_right_max_4() {
        let mut btree = BTree::new(160,"test_files","btree_test_delete_join_right_max_4");
        btree.set_up();
        btree.insert(4, 0 as usize).unwrap();
        btree.insert(3, 1 as usize).unwrap();
        btree.insert(6, 2 as usize).unwrap();
        btree.insert(2, 3 as usize).unwrap();
        btree.insert(1, 4 as usize).unwrap();
        btree.insert(8, 5 as usize).unwrap();
        btree.insert(10, 6 as usize).unwrap();
        let _o:Option<usize> = btree.delete(3);
        let v: Vec<i32> = vec![2,1,2,10,4,6,8,10];
        assert_eq!(btree.traverse::<i32, usize>().unwrap(), v);
        btree.close();
    }

    //delete rechter Nachbar füllt underflow auf
    #[test]
    pub fn test_btree_delete_fill_with_right_node_max_4() {
        let mut btree = BTree::new(160,"test_files","btree_test_delete_fill_with_right_node_max_4");
        btree.set_up();
        btree.insert(4, 0 as usize).unwrap();
        btree.insert(3, 1 as usize).unwrap();
        btree.insert(2, 3 as usize).unwrap();
        btree.insert(1, 4 as usize).unwrap();
        btree.insert(10, 6 as usize).unwrap();
        btree.insert(11, 7 as usize).unwrap();
        btree.insert(12, 7 as usize).unwrap();
        btree.insert(14, 8 as usize).unwrap();

        let _o:Option<usize> = btree.delete(3);
        let v: Vec<i32> = vec![2,1,2,10,4,10,14,11,12,14];
        assert_eq!(btree.traverse::<i32, usize>().unwrap(), v);
        btree.close();
    }

    //delete join mit linkem Nachbar unterlauf
    #[test]
    pub fn test_btree_delete_join_left_max_4() {
        let mut btree = BTree::new(160,"test_files","btree_test_delete_join_left_max_4");
        btree.set_up();
        btree.insert(2, 3 as usize).unwrap();
        btree.insert(1, 4 as usize).unwrap();
        btree.insert(6, 7 as usize).unwrap();
        btree.insert(12, 7 as usize).unwrap();
        btree.insert(14, 8 as usize).unwrap();
        btree.insert(5, 6 as usize).unwrap();
        btree.insert(4, 0 as usize).unwrap();
        btree.insert(3, 1 as usize).unwrap();

        let _o:Option<usize> = btree.delete(6);
        let _o:Option<usize> = btree.delete(12);

        let v: Vec<i32> = vec![2, 1, 2, 14, 3, 4, 5, 14];
        assert_eq!(btree.traverse::<i32, usize>().unwrap(), v);

        btree.close();
    }

    //delete linker Nachbar füllt underflow auf
    #[test]
    pub fn test_btree_delete_fill_with_left_node_max_4() {
        let mut btree = BTree::new(160,"test_files","btree_test_delete_fill_with_left_node_max_4");
        btree.set_up();
        btree.insert(12, 7 as usize).unwrap();
        btree.insert(9, 19 as usize).unwrap();
        btree.insert(11, 21 as usize).unwrap();
        btree.insert(8, 18 as usize).unwrap();
        btree.insert(7, 17 as usize).unwrap();
        btree.insert(1, 4 as usize).unwrap();
        btree.insert(2, 3 as usize).unwrap();
        btree.insert(3, 1 as usize).unwrap();
        btree.insert(4, 0 as usize).unwrap();
        btree.insert(5, 6 as usize).unwrap();
        btree.insert(6, 7 as usize).unwrap();
        btree.insert(10, 20 as usize).unwrap();

        let _o:Option<usize> = btree.delete(9);
        let _o:Option<usize> = btree.delete(10);
        let _o:Option<usize> = btree.delete(11);

        let v: Vec<i32> = vec![2, 1, 2, 4, 3, 4, 7, 5, 6, 7, 12, 8, 12];
        assert_eq!(btree.traverse::<i32, usize>().unwrap(), v);

        btree.close();
    }

    #[test]
    pub fn test_btree_delete_none_exist() {
        let mut btree = BTree::new(500,"test_files","btree_test_delete_none_exist");
        btree.set_up();
        for i in 0..100 {
            let student = Student::new("Witt", "Jonathan", i as usize);
            btree.insert(i ,  student).unwrap();
        }

        let students: Option<Vec<Student>> = btree.delete(999);
        assert_eq!(students.is_none(),true);
        btree.close();
    }

    #[test]
    pub fn test_btree_delete_usize_as_comp() {
        let mut btree = BTree::new(400,"test_files","btree_test_delete_usize_as_comp");
        btree.set_up();
        for i in 0..50 {
            let student = Student::new("Witt", "Jonathan", i as usize);
            btree.insert(student.matrikelnr,  student).unwrap();
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

}