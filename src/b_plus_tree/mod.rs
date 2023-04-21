use crate::node::Node;
use crate::inner_element::InnerElement;
use crate::leaf_element::LeafElement;
use crate::block::Block;
use crate::bfa::BFA;


use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::{Debug, Error};
use std::mem::size_of_val;


pub struct BTree {
    pub root_id:usize,
    pub bfa:BFA,
    pub max:usize,  //maximum Number of Elements per Node
}

impl BTree {
    //constructor- creates BFA
    pub fn new(block_size: usize, dir: &str, filestr: &str) -> BTree {
        let bfa = BFA::new(block_size, dir, filestr);
        BTree{
            root_id: 0,
            bfa,
            max: 0
        }
    }
    //gets informations like max and root
    pub fn set_up(&mut self) {
        self.root_id = self.get_root();
        let max = self.get_max();
        match max {
            Some(string) => {
                let max = string.parse::<usize>().expect("invalid max_string");
                self.max = max;
            }
            None => {
                self.max = 0;
            }
        }
    }

    pub fn search_interval<T :Ord, V> (&mut self, start: T, end: T) -> Option<Vec<V>>
        where T: DeserializeOwned + Debug,
              V: Serialize + DeserializeOwned + Debug + Clone {
        if end < start {
            return None;
        }
        else {
            let vec = self.search_interval_internal(start,end,self.root_id, Vec::new());
            return vec;
        }

    }
    //for testing purposes:
    // Traverse Tree in pre-Order
    // Maybe in the future (starting from @param given Node, writing all traversed Comparators to a Vector) -> this is buggy
    // always start with root_id -> btree.tarverse(btree.root_id)
    pub fn traverse<T :Ord,V>(&mut self) -> Option<Vec<T>>
        where T: DeserializeOwned + Debug + Copy ,
              V: DeserializeOwned + Debug + Clone {

        self.traverse_internal::<T,V>(self.root_id)
    }
    //need @param given for recursion
    pub fn traverse_internal<T :Ord,V>(&mut self, given : usize) -> Option<Vec<T>>
        where T: DeserializeOwned + Debug + Copy ,
              V: DeserializeOwned + Debug + Clone {

        let mut result : Vec<T> = Vec::new();
        let tmp: Node<T,V> = Node::from_block(&mut self.bfa.get(given).expect("from_block error"));
        match tmp {
            Node::LeafNode {content, l_ref: _, r_ref: _} => {
                for i in 0..content.len() {
                    result.push(content.get(i).unwrap().comp);
                }
            }
            Node::InnerNode{content} => {
                for i in 0..content.len() {
                    result.push(content.get(i).unwrap().comp.clone());
                    let mut v :Vec<T> = self.traverse_internal::<T, V>(content.get(i).unwrap().id).unwrap();
                    result.append(&mut v);

                }
            }
        }
        return Some(result);
    }

    pub fn search<T :Ord, V>(&mut self, x: T) -> Option<V>
        where T: DeserializeOwned + Debug,
              V: Serialize + DeserializeOwned + Debug {
        let data = self.search_internal(x, self.root_id);
        return data;
    }

    //search x
    //first call -> given = BTree.root_id
    // prints out passed Nodes + el for debug purposes
    fn search_internal<T, V> (&mut self, x: T, given: usize) -> Option<V>
        where T: DeserializeOwned + Debug + Ord,
              V: DeserializeOwned + Debug + Sized {
        let tmp: Node<T, V> = Node::from_block(&mut self.bfa.get(given).expect("error search internal"));
        match tmp {
            Node::LeafNode {content, l_ref: _, r_ref: _} => {
                for i in content {
                    if x == i.comp {
                        return Some(i.data);
                    }
                }
            }
            Node::InnerNode {content} => {
                for i in content {
                    if i.comp >= x {
                        return self.search_internal(x, i.id);
                    }
                }
            }
        }
        None
    }

    fn search_interval_internal<T, V> (&mut self, start: T, end: T, given: usize, mut result: Vec<V>) -> Option<Vec<V>>
        where T: DeserializeOwned + Debug + Ord,
              V: DeserializeOwned + Debug + Sized + Clone {

        let tmp: Node<T, V> = Node::from_block(&mut self.bfa.get(given).expect("error search internal"));
        match tmp {
            Node::LeafNode {content, l_ref: _, r_ref} => {
                for i in 0..content.len() {
                    let element = content.get(i).expect("error search internal");
                    if element.comp >= start && element.comp <= end {
                        result.push(element.data.clone());
                    }
                }
                if content.last().unwrap().comp <= end {
                    match r_ref {
                        None=> return Some(result),
                        Some(pointer)=> return self.search_interval_internal(start,end,pointer, result)
                    }
                }
                else {
                    return Some(result);
                }
            }
            Node::InnerNode{content} => {
                for i in content {
                    if i.comp >= start {
                        return self.search_interval_internal(start,end, i.id, result);
                    }
                }
            }
        }
        None
    }


    pub fn insert<T, V>(&mut self, key: T, value: V) -> Result<(), Error>
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {

        if self.bfa.reserve_count == 0 && self.max == 0 {
            let value_size = size_of_val(&value);
            let key_size = size_of_val(&key);
            //bincode need ~32 bytes for the information from leafNode and 16 bytes for the calculation of the sizes of comp and value
            let max = (self.bfa.block_size - 32) / (value_size + key_size + 16);
            self.set_max(max);
            let mut node: Node<T, V> = Node::LeafNode{content: Vec::new(), l_ref: None, r_ref: None};
            let block = Block::to_block(&mut node);
            self.bfa.insert(block);
        }

        let search: Option<V> = self.search_internal(key, self.root_id);
        match search {
            Some(_val) => {
                println!("comp {:?} already exists", &key);
                return Err(Error);
            }
            None => {
                let element = LeafElement::new(key, value);
                let res = self.b_tree_insert(element, self.root_id, vec![self.root_id]);
                return res;
            }
        }
    }

    //insert el in Tree
    //if id in tree -> None else -> Error Message
    //first call : given = Node::ROOT_ID
    //passed : alle traversiert Nodes
    //update : alle veränderten Nodes -> Blöcke müssen geupdatet werden
    fn b_tree_insert<T, V> (&mut self, element: LeafElement<T, V>, given: usize, mut passed: Vec<usize>) -> Result<(), Error>
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {

        let tmp: Node<T, V> = Node::from_block(&mut self.bfa.get(given).unwrap());
        match tmp {
            Node::LeafNode {mut content, l_ref, r_ref} => {
                for i in 0..content.len() {
                    if &element.comp < &content.get(i).unwrap().comp {
                        if &content.len() < &self.max {
                            content.insert(i, element);
                            let mut new_tmp = Node::LeafNode {content, l_ref, r_ref };
                            let tmp_block = Block::to_block( &mut new_tmp);
                            self.bfa.update(given, tmp_block).expect("error b_tree_insert");
                            return Ok(());
                        }
                        else { // falls Node voll
                            self.split_leaf(element.clone(), passed, element.data.clone());
                            //self.split_Node(Some(element.clone()), None, &mut passed, element.data.clone());
                            return Ok(());
                        }
                    }
                    else if element.comp == content.get(i).unwrap().comp {
                        return Err(Error);
                    }
                }
                // element größer als alle im Knoten
                if &content.len() < &self.max {
                    content.push( element);
                    let mut new_tmp = Node::LeafNode {content, l_ref, r_ref };
                    let tmp_block = Block::to_block(&mut new_tmp);
                    self.bfa.update(given, tmp_block).expect("error b_tree_insert");
                    Ok(())
                }
                else {
                    self.split_leaf(element.clone(), passed, element.data.clone());
                    //self.split_Node(Some(element.clone()), None, &mut passed, element.data);
                    Ok(())
                }
            }
            Node::InnerNode {mut content} => {
                for i in &content {
                    if element.comp <= i.comp {
                        passed.push(i.id);
                        return self.b_tree_insert(element,i.id ,passed);
                    }
                }
                //el comp größer als alle im Knoten -> letzten Index = el.comp
                let mut tmp_el: InnerElement<T> = content.pop().expect("error b tree insert");
                tmp_el.comp = element.comp.clone();
                passed.push(tmp_el.id);
                content.push(tmp_el);
                let mut new_tmp: Node<T, V> = Node::InnerNode{content };
                let tmp_block = Block::to_block(&mut new_tmp);
                self.bfa.update(given, tmp_block).expect("error btree b_tree_insert");
                return self.b_tree_insert(element,tmp_el.id, passed);
            }
        }
    }

    fn split_leaf <T, V> (&mut self, leaf_element: LeafElement<T, V>, mut passed: Vec<usize>, data: V)
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {

        let current_id = passed.pop().unwrap();
        let tmp: Node<T, V> = Node::from_block(&mut self.bfa.get(current_id).unwrap());
        let mut b = false;
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
                let node2_id = self.bfa.reserve();
                let block2 = Block::to_block(&mut tmp_node2);
                self.bfa.update(node2_id, block2).expect("error btree split leaf");

                //update right neighbor if exists
                match r_ref {
                    Some(id_of_right_neighbor)=>{
                        let mut right_neighbor: Node<T, V> = Node::from_block(&mut self.bfa.get(id_of_right_neighbor ).unwrap());
                        right_neighbor.set_lref(Some(node2_id));
                        let block = Block::to_block(&mut right_neighbor);
                        self.bfa.update(id_of_right_neighbor , block).expect("error btree split leaf");
                    }
                    None=>()

                }
                //update first half
                r_ref = Some(node2_id);
                let mut new_tmp = Node::LeafNode {content: content.clone(), l_ref: l_ref, r_ref: r_ref};
                let block1 = Block::to_block(&mut new_tmp);
                self.bfa.update(current_id, block1).expect("error btree split leaf");

                //update parent
                let new_el_for_parent = InnerElement::new(parent_comp, node2_id);
                if passed.len() != 0 {
                    let id = *passed.last().unwrap();
                    self.adjust_ref(content.last().unwrap().comp, id, data.clone());
                    //Rekursion
                    if !(self.insert_into_node(new_el_for_parent, *passed.last().unwrap(), data.clone())) {
                        self.split_inner(new_el_for_parent, passed, data.clone());
                    }
                }
                else { //make new root-Node
                    let el2_for_root = InnerElement::new(tmp_node2.get_leaf_content().unwrap().last().unwrap().comp, node2_id);
                    let el1_for_root = InnerElement::new(content.last().unwrap().comp, current_id);
                    self.new_root(el1_for_root, el2_for_root, data.clone(), current_id);
                }
            }
            _ => {
                println!("Node {}, was not a LeafNode -> should not happen as LeafNode refs only point to other LeafNodes", current_id); //TODO passed contains Inner -> tests split
            }
        }
    }

    fn split_inner <T, V> (&mut self, inner_element: InnerElement<T>, mut passed: Vec<usize>, data: V)
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {
        let current_id = passed.pop().unwrap();
        let tmp: Node<T, V> = Node::from_block(&mut self.bfa.get(current_id).unwrap());
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
                let node2_id = self.bfa.reserve();
                let new_el_for_parent = InnerElement::new(tmp_node2.get_inner_content().unwrap().last().unwrap().comp, node2_id);
                let block2 = Block::to_block(&mut tmp_node2);
                self.bfa.update(node2_id, block2).expect("error btree split inner");
                let mut new_tmp: Node<T, V> = Node::InnerNode{content: content.clone()};
                let block1 = Block::to_block(&mut new_tmp);
                self.bfa.update(current_id, block1).expect("error btree split inner");

                if passed.len() != 0 {
                    let id = *passed.last().unwrap();
                    self.adjust_ref(content.last().unwrap().comp, id, data.clone());
                    //Rekursion
                    if !(self.insert_into_node(new_el_for_parent, *passed.last().unwrap(), data.clone())) {
                        self.split_inner(new_el_for_parent, passed, data.clone());
                    }
                }
                else { //make new root-Node
                    let el2_for_root = InnerElement::new(tmp_node2.get_inner_content().unwrap().last().unwrap().comp, node2_id);
                    let el1_for_root = InnerElement::new(content.last().unwrap().comp, current_id);
                    self.new_root(el1_for_root, el2_for_root, data.clone(), current_id);
                }
            }
            _ => {
                println!("Something went wrong splitting Node {}", current_id);
            }
        }
    }

    //fügt neuen comp in Node mit id = id ein
    fn adjust_ref<T, V> (&mut self, comp: T, id: usize, _data: V)
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {
        let mut tmp: Node<T, V> = Node::from_block(&mut self.bfa.get(id).unwrap());
        for i in 0..tmp.get_inner_content().unwrap().len(){
            if &comp < &tmp.get_inner_content().unwrap().get(i).unwrap().comp {
                tmp.get_inner_content().unwrap()[i].comp = comp;
                break;
            }
        }
        let block = Block::to_block(&mut tmp);
        self.bfa.update(id,block).expect("error btree adjust ref");
    }

    fn new_root<T, V> (&mut self, el1: InnerElement<T>, el2: InnerElement<T>, _data: V, current_id: usize)
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {
        if current_id != self.root_id {
            println!("passed[0] is not root !? Something went wrong");
            return;
        }
        let mut new_root_node: Node<T, V> = Node::InnerNode{content: vec![el1, el2]};
        let root_id = self.bfa.reserve();
        self.set_root(root_id);
        let block = Block::to_block(&mut new_root_node);
        self.bfa.update(self.root_id, block).expect("error btree new root");
    }
    fn insert_into_node<T, V> (&mut self, el : InnerElement<T>, id : usize, _data: V) -> bool
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized {
        let tmp: Node<T, V> = Node::from_block(&mut self.bfa.get(id).unwrap());
        match tmp {
            Node::InnerNode {mut content} => {
                if content.len() < self.max {
                    for i in 0..content.len() {
                        if el.comp < content.get(i).unwrap().comp {
                            content.insert(i, el);
                            break;
                        }
                        if i == content.len() - 1 {
                            content.push(el);
                        }
                    }
                    let mut new_tmp: Node<T, V> = Node::InnerNode {content};
                    let block = Block::to_block(&mut new_tmp);
                    self.bfa.update(id,block).expect("error btree insert into Node");
                    return true
                }
                else {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
    }

    //Print the Structure of Tree for Debugging -> starting_point can be any existing id
    // Turbofish notation
    // Give a Node id Some(id) as arbitrary start Point or None to start at root
    pub fn print<T, V>(&mut self, _starting_point: Option<T>)
            where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
                  V: Serialize + DeserializeOwned + Debug + Sized + Clone {
        /*TODO Idea:
        match starting_point {
            Some(id)=>self.print_tree_structure::<T, V>(id),
            None=>self.print_tree_structure::<T, V>(self.root_id)
        }
        TODO but root_id is usize not T and is initialized from metadata file => parse from metadata with match type { "usize"=> parse::<usize> ...
        */
        self.print_tree_structure::<T, V>(self.root_id);
        println!();
    }

    fn print_tree_structure<T, V>(&mut self, given: usize)
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {

        println!();
        let tmp: Node<T, V> = Node::from_block(&mut self.bfa.get(given).unwrap());
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
                    self.print_tree_structure::<T, V>(i.id);
                }
            }
        }
    }

    pub fn delete<T, V> (&mut self, x: T) -> Option<V>
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {
        let data = self.delete_internal(x, self.root_id, Vec::new());
        return data;
    }

    fn delete_internal<T, V>(&mut self, key: T, given: usize, mut passed: Vec<usize>) -> Option<V>
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {
        let tmp: Node<T, V> = Node::from_block(&mut self.bfa.get(given).unwrap());
        match tmp {
            Node::LeafNode {mut content, l_ref, r_ref} => {
                for i in 0..content.len() {
                    let comp = &content.get(i).expect("delete: something went wrong").comp;
                    if &key == comp  {
                        let data = content.remove(i).data;
                        let mut new_tmp: Node<T, V> = Node::LeafNode {content: content.clone(), l_ref: l_ref, r_ref: r_ref};
                        let block = Block::to_block(&mut new_tmp);
                        self.bfa.update(given, block).expect("error btree delete internal");
                        if i == content.len() {
                            for i in 0..passed.len() {
                                let parent_id = passed.get(i).expect("error delete internal").clone();
                                let mut parent: Node<T, V> = Node::from_block(&mut self.bfa.get(parent_id).unwrap());
                                let newcomp = content.last().expect("error delete internal").comp.clone();
                                let parent_content = parent.get_inner_content().expect("error delete internal");
                                for i in 0..parent_content.len() {
                                    let el = parent_content.get(i).unwrap().comp;
                                    if el == key {
                                        parent_content[i].comp = newcomp;
                                    }
                                }
                                let block2 = Block::to_block(&mut parent);
                                self.bfa.update(parent_id, block2).expect("error btree delete internal");
                            }
                        }
                        if content.len() < self.max/2 as usize && !passed.is_empty() {
                            self.join_node(key, given, passed, data.clone());
                        }
                        return Some(data);
                    }
                }
            }
            Node::InnerNode {content} => {
                for i in content {
                    if i.comp >= key {
                        passed.push(given);
                        return self.delete_internal(key, i.id, passed);
                    }
                }
            }
        }
        return None;
    }

    fn join_node<T, V>(&mut self, key: T, given: usize, mut passed: Vec<usize>, data: V)
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {
        let parent_id = passed.pop().expect("join Node error");
        let mut underflow: Node<T, V> = Node::from_block(&mut self.bfa.get(given).unwrap());
        let mut parent: Node<T, V> = Node::from_block(&mut self.bfa.get(parent_id).unwrap());
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
        let neighbor: Node<T, V> = Node::from_block(&mut self.bfa.get(neighbor_block_id).unwrap());
        match neighbor {
            Node::LeafNode {content, l_ref, r_ref} => {
                //neighbor ist zur hälfte gefüllt
                if content.len()  <= self.max/2 as usize {
                    //rechter neighbor ist zur hälfte gefüllt
                    if right {
                        self.join_leaf(underflow, content, l_ref, r_ref, neighbor_block_id, right);
                        self.fix_parent(parent, parent_id, neighbor_block_id, neighbor_index_in_parent, passed, data, right, key);
                    }
                    //linker neighbor ist zur hälfte gefüllt
                    else {
                        self.join_leaf(underflow, content, l_ref, r_ref, given, right);
                        self.fix_parent(parent, parent_id, given, neighbor_index_in_parent, passed, data, right, key);
                    }
                }
                else {
                    let underflow_content = underflow.get_leaf_content().expect("error join Node");
                    //underflow und neighbor können verschmolzen werden
                    if underflow_content.len() + content.len() <= self.max {
                        //neighbor ist rechts
                        if right {
                            self.join_leaf(underflow, content, l_ref, r_ref, neighbor_block_id, right);
                            self.fix_parent(parent, parent_id, neighbor_block_id, neighbor_index_in_parent, passed, data, right, key);
                        }
                        //neighbor ist links
                        else {
                            self.join_leaf(underflow, content, l_ref, r_ref, given, right);
                            self.fix_parent(parent, parent_id, given, neighbor_index_in_parent, passed, data, right, key);
                        }
                    }
                    //underflow und neighbor können nicht verschmolzen werden, nehme dann wenn möglich den rechten nachbar und füllt underflow damit auf
                    else {
                        //neighbor ist rechts
                        if right{
                            self.fill_leaf(underflow, content, l_ref, r_ref, parent, given, neighbor_block_id, parent_id, neighbor_index_in_parent, right);
                        }
                        //neighbor ist links
                        else {
                            self.fill_leaf(underflow, content, l_ref, r_ref, parent, given, neighbor_block_id, parent_id, neighbor_index_in_parent, right);
                        }
                    }
                }
            }
            Node::InnerNode {content} => {
                //neighbor ist zur hälfte gefüllt
                if content.len()  <= self.max/2 as usize {
                    //rechter neighbor ist zur hälfte gefüllt
                    if right {
                        self.join_inner(underflow, content, neighbor_block_id, right);
                        self.fix_parent(parent, parent_id, neighbor_block_id, neighbor_index_in_parent, passed, data, right, key);
                    }
                    //linker neighbor ist zur hälfte gefüllt
                    else {
                        self.join_inner(underflow, content, given, right);
                        self.fix_parent(parent, parent_id, given, neighbor_index_in_parent, passed, data, right, key);
                    }
                }
                else {
                    let underflow_content = underflow.get_inner_content().expect("error join Node");
                    //underflow und neighbor können verschmolzen werden
                    if underflow_content.len() + content.len() <= self.max {
                        //neighbor ist rechts
                        if right {
                            self.join_inner(underflow, content, neighbor_block_id, right);
                            self.fix_parent(parent, parent_id, neighbor_block_id, neighbor_index_in_parent, passed, data, right, key);
                        }
                        //neighbor ist links
                        else {
                            self.join_inner(underflow, content, given, right);
                            self.fix_parent(parent, parent_id, given, neighbor_index_in_parent, passed, data, right, key);
                        }
                    }
                    //underflow und neighbor können nicht verschmolzen werden, nehme dann wenn möglich den rechten nachbar und füllt underflow damit auf
                    else {
                        //neighbor ist rechts
                        if right {
                            self.fill_inner(underflow, content, parent, given, neighbor_block_id, parent_id, neighbor_index_in_parent, right);
                        }
                        //neighbor ist links
                        else {
                            self.fill_inner(underflow, content, parent, given, neighbor_block_id, parent_id, neighbor_index_in_parent, right);
                        }
                    }
                }
            }
        }
    }

    fn fix_parent<T, V>(&mut self, mut parent: Node<T, V>, parent_id: usize, neighbor_block_id: usize, neighbor_index_in_parent: usize, passed: Vec<usize>, data: V, right: bool, mut key: T)
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {
        let parent_content = parent.get_inner_content().expect("error join Node");
        if right {
            parent_content.remove(neighbor_index_in_parent - 1 );
            let parent_content_len = parent_content.len();
            let block2 = Block::to_block(&mut parent);
            self.bfa.update(parent_id, block2).expect("error btree fix parent");
            if parent_content_len < self.max / 2 as usize {
                if !passed.is_empty() {
                    let parent_parent_id = passed.last().expect("Node join error parent parent").clone();
                    let mut parent_parent: Node<T, V> = Node::from_block(&mut self.bfa.get(parent_parent_id).unwrap());
                    let parent_parent_content = parent_parent.get_inner_content().expect("error join Node");
                    for i in parent_parent_content {
                        if i.id == parent_id {
                            key = i.comp.clone();
                            break;
                        }
                    }
                    self.join_node(key, parent_id, passed, data);
                }
                if parent_id == self.root_id && parent_content_len < 2 {
                    let mut new_parent: Node<T, V> = Node::from_block(&mut self.bfa.get(parent_id).unwrap());
                    if new_parent.get_inner_content().expect("error join Node").len() < 2 {
                        self.set_root(neighbor_block_id);
                    }
                }
            }
        }
        else {
            parent_content.remove(neighbor_index_in_parent);
            let parent_content_len = parent_content.len();
            let block2 = Block::to_block(&mut parent);
            self.bfa.update(parent_id, block2).expect("error btree fix parent");
            if parent_content_len < self.max / 2 as usize {
                if !passed.is_empty() {
                    let parent_parent_id = passed.last().expect("Node join error parent parent").clone();
                    let mut parent_parent: Node<T, V> = Node::from_block(&mut self.bfa.get(parent_parent_id).unwrap());
                    let parent_parent_content = parent_parent.get_inner_content().expect("error join Node");
                    for i in parent_parent_content {
                        if i.id == parent_id {
                            key = i.comp.clone();
                            break;
                        }
                    }
                    self.join_node(key, parent_id, passed, data);
                }
                if parent_id == self.root_id && parent_content_len < 2 {
                    let mut new_parent: Node<T, V> = Node::from_block(&mut self.bfa.get(parent_id).unwrap());
                    if new_parent.get_inner_content().expect("error join Node").len() < 2 {
                        self.set_root(neighbor_block_id);
                    }
                }
            }
        }
    }

    fn join_leaf<T, V>(&mut self, mut underflow: Node<T, V>, mut content: Vec<LeafElement<T, V>>, mut l_ref: Option<usize>, r_ref: Option<usize>, id_for_neighbor: usize, right: bool)
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {

        let underflow_content = underflow.get_leaf_content().expect("error join Node");
        if right {
            underflow_content.extend(content);
            content = underflow_content.clone();
            l_ref = underflow.get_lref();

            match l_ref {   //set left Neighbors rref to the new, merged Node
                Some(left_ref)=> {
                    let mut other_neighbor: Node<T, V> = Node::from_block(&mut self.bfa.get(left_ref).unwrap());
                    other_neighbor.set_rref(Some(id_for_neighbor));
                    let block = Block::to_block(&mut other_neighbor);
                    self.bfa.update(left_ref, block).expect("error btree join leaf");
                },
                None=>()
            }

            let mut new_neighbor = Node::LeafNode {content, l_ref, r_ref };
            let block1 = Block::to_block(&mut new_neighbor);
            self.bfa.update(id_for_neighbor, block1).expect("error btree join leaf");
        }
        else {  //Set right Neighbors lref to new, merged Node
            content.extend(underflow_content.clone());
            underflow.set_leaf_content(content.clone());
            underflow.set_lref(l_ref);
            match l_ref {                       //TODO THis cant be right -> should check r_ref and set right neighbors l_ref to new merged Node -> not the l_ref in the Signature? -> Test a join Node with left Neighbor
                Some(left_ref)=> {
                    let mut other_neighbor: Node<T, V> = Node::from_block(&mut self.bfa.get(left_ref).unwrap());
                    other_neighbor.set_rref(Some(id_for_neighbor));
                    let block = Block::to_block(&mut other_neighbor);
                    self.bfa.update(left_ref, block).expect("error btree join leaf");
                },
                None=>()
            }
            let block1 = Block::to_block(&mut underflow);
            self.bfa.update(id_for_neighbor, block1).expect("error btree join leaf");
        }
    }

    fn join_inner<T, V>(&mut self, mut underflow: Node<T, V>, mut content: Vec<InnerElement<T>>, id_for_neighbor: usize, right: bool)
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {
        let underflow_content = underflow.get_inner_content().expect("error join Node");

        if right {
            underflow_content.extend(content);
            content = underflow_content.clone();
            let mut new_neighbor: Node<T, V> = Node::InnerNode {content };
            let block1 = Block::to_block(&mut new_neighbor);
            self.bfa.update(id_for_neighbor, block1).expect("error btree join inner");
        }
        else {
            content.extend(underflow_content.iter());
            underflow.set_inner_content(content.clone());
            let block1 = Block::to_block(&mut underflow);
            self.bfa.update(id_for_neighbor, block1).expect("error btree join inner");
        }
    }

    fn fill_leaf<T, V>(&mut self, mut underflow: Node<T, V>, mut content: Vec<LeafElement<T, V>>, l_ref: Option<usize>, r_ref: Option<usize>, mut parent: Node<T, V>, given: usize, neighbor_block_id: usize, parent_id: usize, neighbor_index_in_parent: usize, right: bool)
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {
        let parent_content = parent.get_inner_content().expect("error fill leaf");

        if right {
            let underflow_content = underflow.get_leaf_content().expect("error fill leaf");
            while underflow_content.len() < self.max / 2 as usize {
                let a = content.first().expect("error fill leaf").clone();
                content.remove(0);
                underflow_content.push(a);
            }
            let comp = underflow_content.last().expect("error join Node").comp.clone();
            parent_content.get_mut(neighbor_index_in_parent-1).unwrap().comp = comp;

        }
        else {
            let underflow_content = underflow.get_leaf_content().unwrap();
            while underflow_content.len() < self.max / 2 as usize {
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
        self.bfa.update(given, block1).expect("error fill leaf");
        self.bfa.update(neighbor_block_id, block2).expect("error fill leaf");
        self.bfa.update(parent_id, block3).expect("error fill leaf");
    }

    fn fill_inner<T, V>(&mut self, mut underflow: Node<T, V>, mut content: Vec<InnerElement<T>>, mut parent: Node<T, V>, given: usize, neighbor_block_id: usize, parent_id: usize, neighbor_index_in_parent: usize, right: bool)
        where T: Serialize + DeserializeOwned + Debug + Ord + Copy + Clone,
              V: Serialize + DeserializeOwned + Debug + Sized + Clone {
        let parent_content = parent.get_inner_content().expect("error fill inner");
        let underflow_content = underflow.get_inner_content().expect("error fill inner");
        if right {
            while underflow_content.len() < self.max / 2 as usize {
                let a = content.first().expect("error fill inner").clone();
                content.remove(0);
                underflow_content.push(a);
            }
            let comp = underflow_content.last().expect("error fill inner").comp.clone();
            parent_content.get_mut(neighbor_index_in_parent-1).expect("error join Node").comp = comp;
        }
        else {
            while underflow_content.len() < self.max / 2 as usize {
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
        self.bfa.update(given, block1).expect("error fill inner");
        self.bfa.update(neighbor_block_id, block2).expect("error fill inner");
        self.bfa.update(parent_id, block3).expect("error fill inner");
    }

    pub fn set_root(&mut self, root: usize) {
        self.root_id = root;
        self.bfa.metadata_file.insert("root".to_string(), root.to_string());
    }

    pub fn set_max(&mut self, max: usize) {
        self.max = max;
        self.bfa.metadata_file.insert("max".to_string(), max.to_string());
    }

    pub fn get_root(&mut self) -> usize {
        let root_string = self.bfa.metadata_file.get("root").expect("no such string");
        let root = root_string.parse::<usize>().expect("invalid root_string");
        return root;
    }

    pub fn get_max(&mut self) -> Option<&String> {
        let max_string = self.bfa.metadata_file.get("max");
        return max_string;
    }

    pub fn close(&mut self) {
        self.set_root(self.root_id);
        self.set_max(self.max);
        self.bfa.close().expect("error btree close");
    }
}



