use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::Debug;
use std::cmp::{Ord, Ordering};

use crate::base::{TreeNode, Tree};

type RcRefcellBSNode<T>= Rc<RefCell<BSTreeNode<T>>>;
type OptionNode<T>= Option<RcRefcellBSNode<T>>;

pub struct BSTreeNode<T: Ord + Copy + Debug> {
    key: T,
    left: OptionNode<T>,
    right: OptionNode<T>,
}

pub struct BSTree<T: Ord + Copy + Debug> {
    root: OptionNode<T>
}

impl<T: Ord + Copy + Debug> TreeNode<T> for BSTreeNode<T> {
    fn get_left(&self) -> &OptionNode<T> {
        &self.left
    }

    fn get_right(&self) -> &OptionNode<T> {
        &self.right
    }

    fn get_data(&self) -> T {
        self.key
    }
}

impl <T: Ord + Copy + Debug> BSTreeNode<T> {
    fn new(value: T) -> BSTreeNode<T> {
        BSTreeNode {
            key: value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        if self.key == value {
            println!("The node already exists.");
            return
        }
        let new_node =
            if value < self.key {&mut self.left}
            else {&mut self.right};
        match new_node {
            Some(node) => node.borrow_mut().insert(value),
            None => {
                *new_node = Some(Rc::new(RefCell::new(BSTreeNode::new(value))));
            }
        }
    }

    fn delete(&mut self, value: T) {
        match self.key.cmp(&value) {
            Ordering::Greater => self._delete_left(value),
            Ordering::Less => self._delete_right(value),
            _ => unreachable!(),
        }
    }

    fn _delete_node_have_two_children(left: &RcRefcellBSNode<T>) {
        let right_min = left.borrow().right.as_ref().unwrap().borrow().get_min();
        left.borrow_mut().delete(right_min);
        left.borrow_mut().key = right_min;
    }

    fn _delete_right(&mut self, value: T) {
        if let Some(right) = self.right.as_ref() {
            if right.borrow().key == value {
                if right.borrow().left.is_none() && right.borrow().right.is_none() {
                    self.right = None;
                } else if right.borrow().left.is_none() && !right.borrow().right.is_none() {
                    self.right.take().map(|node| {
                        self.right = node.borrow().right.clone()
                    });
                } else if !right.borrow().left.is_none() && right.borrow().right.is_none() {
                    self.right.take().map(|node| {
                        self.right = node.borrow().left.clone()
                    });
                } else {
                    Self::_delete_node_have_two_children(right);
                }
            } else {
                right.borrow_mut().delete(value);
            }
        }
    }

    fn _delete_left(&mut self, value: T) {
        if let Some(left) = self.left.as_ref() {
            if left.borrow().key == value {
                if left.borrow().left.is_none() && left.borrow().right.is_none() {
                    self.left = None;
                } else if left.borrow().left.is_none() && !left.borrow().right.is_none() {
                    self.left.take().map(|node| {
                        self.left = node.borrow().right.clone()
                    });
                } else if !left.borrow().left.is_none() && left.borrow().right.is_none() {
                    self.left.take().map(|node| {
                        self.left = node.borrow().left.clone()
                    });
                } else {
                    Self::_delete_node_have_two_children(left);
                }
            } else {
                left.borrow_mut().delete(value);
            }
        }
    }

    fn print_node(&self, prefix_space: &String, child_prefix: String, is_right: bool) {
        match prefix_space.len() {
            6 => println!("|____ {} {:?}", child_prefix, self.key),
            _ => println!("{}{} {:?}", prefix_space, child_prefix, self.key),
        }
        let mut new_prefix_space: String = String::from(prefix_space);
        match is_right {
            false => {
                match prefix_space.len() {
                    0 => new_prefix_space.push_str("|     "),
                    _ => new_prefix_space.replace_range(
                        prefix_space.len() - 6..prefix_space.len(),
                        "|     |____ ",
                    ),
                }
            }
            true => new_prefix_space.replace_range(
                prefix_space.len() - 6..prefix_space.len(),
                "      |____ ",
            ),
        }
        if let Some(left) = self.get_left() {
            left.borrow().print_node(
                &new_prefix_space,
                "L".to_string(),
                false,
            );
        }
        if let Some(right) = self.get_right() {
            right.borrow().print_node(
                &new_prefix_space,
                "R".to_string(),
                true,
            );
        }
    }
}

impl <T: Ord + Copy + Debug> Tree<T, BSTreeNode<T>> for BSTree<T> {
    fn get_root(&self) -> &OptionNode<T> {
        &self.root
    }

    fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Some(Rc::new(RefCell::new(BSTreeNode::new(value))));
        } else {
            self.root.as_ref().unwrap().borrow_mut().insert(value);
        }
    }

    fn delete(&mut self, value:T){
        if self.root.is_none()||!self.contain(value) {
            println!("The node of value {:#?} doesn't exist.",value);
            return ;
        }else {
            if let Some(root) = self.root.as_ref() {
                if root.borrow().key == value {
                    if root.borrow().left.is_none() && root.borrow().right.is_none() {
                        self.root = None;
                    } else if root.borrow().left.is_none() && !root.borrow().right.is_none() {
                        self.root.take().map(|node| {
                            self.root = node.borrow().right.clone()
                        });
                    } else if !root.borrow().left.is_none() && root.borrow().right.is_none() {
                        self.root.take().map(|node| {
                            self.root = node.borrow().left.clone()
                        });
                    } else {
                        BSTreeNode::_delete_node_have_two_children(root);
                    }
                } else {
                    root.borrow_mut().delete(value);
                }
            }
        }
    }

    fn print_tree(&self) {
        match &self.get_root() {
            None => println!("This tree is empty!"),
            Some(root) => root.borrow().print_node(
                &"".to_string(),
                "Root".to_string(),
                false,
            ),
        }
    }
}

impl<T:Ord + Copy +Debug> BSTree<T> {
    pub fn new() -> Self {
        BSTree {
            root:None,
        }
    }
    
    pub fn clear(&mut self) {
        *self = BSTree::new();
        println!("Clear operation is complete!");
    }
}
