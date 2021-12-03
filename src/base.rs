use std::rc::Rc;
use std::fmt::Debug;
use std::cell::RefCell;
use std::cmp::{max, Ordering};

pub trait TreeNode<T: Ord + Copy + Debug> {
    fn get_left(&self) -> &Option<Rc<RefCell<Self>>>;

    fn get_right(&self) -> &Option<Rc<RefCell<Self>>>;

    fn get_data(&self) -> T;

    fn get_height(&self) -> u32 {
        match (self.get_left(), self.get_right()) {
            (Some(left), Some(right)) => {
                max(left.borrow().get_height(), right.borrow().get_height()) + 1
            }
            (Some(left), _) => left.borrow().get_height() + 1,
            (_, Some(right)) => right.borrow().get_height() + 1,
            (_, _) => 1,
        }
    }

    fn get_min(&self) -> T {
        match self.get_left() {
            None => self.get_data(),
            Some(left) => left.borrow().get_min(),
        }
    }

    fn get_max(&self) -> T {
        match self.get_right() {
            None => self.get_data(),
            Some(right) => right.borrow().get_max(),
        }
    }

    fn count_leaves(&self) -> u32 {
        match (self.get_left(), self.get_right()) {
            (Some(left), Some(right)) => {
                left.borrow().count_leaves() + right.borrow().count_leaves()
            }
            (Some(left), _) => left.borrow().count_leaves(),
            (_, Some(right)) => right.borrow().count_leaves(),
            (_, _) => 1,
        }
    }

    fn count_nodes(&self) -> u32 {
        match (self.get_left(), self.get_right()) {
            (Some(left), Some(right)) => {
                left.borrow().count_nodes() + right.borrow().count_nodes() + 1
            }
            (Some(left), _) => left.borrow().count_nodes() + 1,
            (_, Some(right)) => right.borrow().count_nodes() + 1,
            (_, _) => 1,
        }
    }

    fn traverse_inorder(&self) {
        if let Some(left) = self.get_left() {
            left.borrow().traverse_inorder();
        }
        println!("{:#?}", self.get_data());
        if let Some(right) = self.get_right() {
            right.borrow().traverse_inorder();
        }
    }

    fn traverse_preorder(&self) {
        println!("{:#?}", self.get_data());
        if let Some(left) = self.get_left() {
            left.borrow().traverse_inorder();
        }
        if let Some(right) = self.get_right() {
            right.borrow().traverse_inorder();
        }
    }

    fn traverse_postorder(&self) {
        if let Some(left) = self.get_left() {
            left.borrow().traverse_inorder();
        }
        if let Some(right) = self.get_right() {
            right.borrow().traverse_inorder();
        }
        println!("{:#?}", self.get_data());
    }

    fn contains(&self, value: T) -> bool {
        match self.get_data().cmp(&value) {
            Ordering::Greater => {
                match self.get_left() {
                    None => false,
                    Some(left) => left.borrow().contains(value),
                }
            }
            Ordering::Less => {
                match self.get_right() {
                    None => false,
                    Some(right) => right.borrow().contains(value),
                }
            }
            Ordering::Equal => true,
        }
    }
}

pub trait Tree<T: Ord + Copy + Debug, TN: TreeNode<T>> {
    fn get_root(&self) -> &Option<Rc<RefCell<TN>>>;

    fn insert(&mut self, data: T);

    fn delete(&mut self, data: T);

    fn print_tree(&self);

    fn get_height(&self) -> u32 {
        match &self.get_root() {
            None => 0,
            Some(node) => node.borrow().get_height(),
        }
    }

    fn get_min(&self) -> Option<T> {
        match &self.get_root() {
            None => None,
            Some(node) => Some(node.borrow().get_min()),
        }
    }

    fn get_max(&self) -> Option<T> {
        match &self.get_root() {
            None => None,
            Some(node) => Some(node.borrow().get_max()),
        }
    }

    fn count_leaves(&self) -> u32 {
        match &self.get_root() {
            None => 0,
            Some(node) => node.borrow().count_leaves(),
        }
    }

    fn count_nodes(&self) -> u32 {
        match &self.get_root() {
            None => 0,
            Some(node) => node.borrow().count_nodes(),
        }
    }

    fn traverse_inorder(&self) {
        match &self.get_root() {
            None => return ,
            Some(node) => node.borrow().traverse_inorder(),
        }
    }

    fn traverse_preorder(&self) {
        match &self.get_root() {
            None => return ,
            Some(node) => node.borrow().traverse_preorder(),
        }
    }

    fn traverse_postorder(&self) {
        match &self.get_root() {
            None => return ,
            Some(node) => node.borrow().traverse_postorder(),
        }
    }

    fn contains(&self, value: T) -> bool {
        match &self.get_root() {
            None => false,
            Some(node) => node.borrow().contains(value),
        }
    }

    fn is_empty(&self) -> bool {
        match &self.get_root() {
            Some(_) => false,
            None => true,
        }
    }
}
