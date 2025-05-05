use std::cell::RefCell;
use std::rc::{Rc, Weak};

use super::tree::{Node, NodeLink};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        //default if current node is NIL
        None
    }

    //Iterative insert Function
    pub fn tree_insert_iterative(current_node_link: &BstNodeLink, value: i32) {
        let z_node = BstNode::new_bst_nodelink(value);
        let mut y_node: Option<Rc<RefCell<BstNode>>> = None;
        let mut x_node: Option<Rc<RefCell<BstNode>>> = Some(Rc::clone(current_node_link)); 
        let z_value = z_node.borrow().key.unwrap();
        
        loop {
            let current_node = match x_node {
                Some(ref node) => Rc::clone(node),
                None => break,
            };
            let current_rc = current_node.borrow();
            
            y_node = Some(Rc::clone(&current_node));

            if let Some(x_value) = current_rc.key {
                if z_value < x_value {
                    x_node = current_rc.left.clone();
                } else {
                    x_node = current_rc.right.clone();
                }
            } else {
                break;
            }
        }
        
        if let Some(parent_node_rc) = y_node {
            let mut x_parent = parent_node_rc.borrow_mut();
            let y_value = x_parent.key.unwrap();

            println!("Inserting {} under parent {}", z_value, y_value);
            
            if z_value < y_value {
                println!("Going left");
                x_parent.left = Some(Rc::clone(&z_node));
            z_node.borrow_mut().parent = Some(BstNode::downgrade(&parent_node_rc));
            } else {
                println!("Going right");
                x_parent.right = Some(Rc::clone(&z_node));
            z_node.borrow_mut().parent = Some(BstNode::downgrade(&parent_node_rc));
            }
        }
    }

    //Transplant
    pub fn transplant(unode: &BstNodeLink, vnode: &BstNodeLink) {
        let parent_opt = {
            let unode_ref = unode.borrow();
            unode_ref.parent.clone()
        };

        if let Some(parent_weak) = parent_opt {
            if let Some(parent_rc) = parent_weak.upgrade() {
                let mut parent_node = parent_rc.borrow_mut();
    

                if let Some(left_child) = &parent_node.left {
                    if Rc::ptr_eq(left_child, unode) {
                        parent_node.left = Some(Rc::clone(vnode));
                    } else {
                        parent_node.right = Some(Rc::clone(vnode));
                    }
                } else {
                    parent_node.right = Some(Rc::clone(vnode));
                }

                vnode.borrow_mut().parent = Some(Rc::downgrade(&parent_rc));
            }
        } else {
            let vnode_borrow = vnode.borrow();
            let mut unode_borrow = unode.borrow_mut();

            unode_borrow.key = vnode_borrow.key.clone();
            unode_borrow.left = vnode_borrow.left.clone();
            unode_borrow.right = vnode_borrow.right.clone();

            if let Some(ref left) = unode_borrow.left {
                left.borrow_mut().parent = Some(Rc::downgrade(unode));
            }

            if let Some(ref right) = unode_borrow.right {
                right.borrow_mut().parent = Some(Rc::downgrade(unode));
            }
    
            unode_borrow.parent = None;
        }
    }

    pub fn deletion(current_node_link: &BstNodeLink, value: i32) {
        let z_node_opt = {
            let root_ref = current_node_link.borrow();
            BstNode::tree_search(&root_ref, &value)
        };

        if let Some(z_node) = z_node_opt {
            let z_node_left = z_node.borrow().left.clone();
            let z_node_right = z_node.borrow().right.clone();

            println!("Deleting node with value of {}...", value);

            if z_node_left.is_none() && z_node_right.is_none() {
                let parent_opt = z_node.borrow().parent.clone();
                if let Some(parent_weak) = parent_opt {
                    if let Some(parent_rc) = parent_weak.upgrade() {
                        let mut parent_node = parent_rc.borrow_mut();
                        if Rc::ptr_eq(&parent_node.left.as_ref().unwrap(), &z_node) {
                            parent_node.left = None;
                        } else {
                            parent_node.right = None;
                        }
                    }
                }
                println!("Deletion Success!");
            } else if z_node_left.is_none() {
                if let Some(right_node) = z_node_right {
                    BstNode::transplant(&z_node, &right_node);
                    println!("Deletion Success!");
                }
            } else if z_node_right.is_none() {
                if let Some(left_node) = z_node_left {
                    BstNode::transplant(&z_node, &left_node);
                }
                println!("Deletion Success!");
            } else {
                if let Some(right_child) = z_node_right {
                    let successor = BstNode::minimum(&right_child.borrow());

                    if let Some(successor_left) = successor.borrow().left.clone() {
                        BstNode::transplant(&successor, &successor_left);
                    }

                    BstNode::transplant(&z_node, &successor);

                    successor.borrow_mut().left = z_node.borrow().left.clone();
                    if let Some(ref left_node) = z_node.borrow().left {
                        left_node.borrow_mut().parent = Some(Rc::downgrade(&successor));
                    }

                    successor.borrow_mut().right = z_node.borrow().right.clone();
                    if let Some(ref right_node) = z_node.borrow().right {
                        right_node.borrow_mut().parent = Some(Rc::downgrade(&successor));
                    }
                    println!("Deletion Success!");
                }
            }
        }
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

        /**
     * Return the root of a node, but this time it didn't clone
     */
    fn get_root_no_clone(node: &BstNodeLink) -> BstNodeLink {
        match BstNode::upgrade_weak_to_strong(node.borrow().parent.clone()) {
            None => Rc::clone(node),
            Some(parent) => BstNode::get_root(&parent),
        }
    }

    /**
     * NOTE: Buggy from pull request
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        // directly check if the node has a right child, otherwise go to the next block
        if let Some(right_node) = &x_node.borrow().right {
            return Some(right_node.borrow().minimum());
        } 
        
        // empty right child case
        else { 
            let mut x_node = x_node;
            let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
            let mut temp: BstNodeLink;

            while let Some(ref exist) = y_node {
                if let Some(ref left_child) = exist.borrow().left {
                    if BstNode::is_node_match(left_child, x_node) {
                        return Some(exist.clone());
                    }
                }

                temp = y_node.unwrap();
                x_node = &temp;
                y_node = BstNode::upgrade_weak_to_strong(temp.borrow().parent.clone());
            }

            None    
        }
    }

    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink>{
        //create a shadow of x_node so it can mutate
        let mut x_node = x_node;
        let right_node = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node)!=true{
            return Some(right_node.clone().unwrap().borrow().minimum());
        }

        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right = &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;
        while BstNode::is_nil(&y_node) && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone()) {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent = y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }

        //in case our sucessor traversal yield root, means self is the highest key
        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return None;
        }

        //default return self / x_node
        return Some(y_node.clone().unwrap())
    }

    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

     /**
     * private function return true if node doesn't have any key in it
     */
    fn is_nil_key(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(rc_node) => rc_node.borrow().key.is_none(),
        }
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            //Some(x) => Some(x.upgrade().unwrap()),

            //fixed weak_to_Strong since it causes error when unwrapping the node
            Some(x) => x.upgrade(),
        }
    }
}
