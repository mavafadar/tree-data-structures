use std::rc::Rc;
use std::cmp::max;
use std::fmt::Debug;
use std::cell::RefCell;
use std::cmp::Ordering;

use crate::base::{TreeNode, Tree};

type RcRefcellAVLNode<T> = Rc<RefCell<AVLTreeNode<T>>>;
type OptionNode<T> = Option<RcRefcellAVLNode<T>>;

/// AVLTreeNode is a node in the Tree (The base moduel).
/// data is the value of the node.
/// _height is the height of the node.
#[derive(Debug)]
pub struct AVLTreeNode<T: Ord + Copy + Debug> {
    pub data: T,
    _left: OptionNode<T>,
    _right: OptionNode<T>,
    _height: u32,
}

impl<T: Ord + Copy + Debug> TreeNode<T> for AVLTreeNode<T> {
    fn get_left(&self) -> &OptionNode<T> {
        &self._left
    }

    fn get_right(&self) -> &OptionNode<T> {
        &self._right
    }

    fn get_data(&self) -> T {
        self.data
    }
}

impl<T: Ord + Copy + Debug> AVLTreeNode<T> {
    /// Makes a new empty AVLTree.
    ///
    /// Does not allocate anything on its own.
    fn new(data: T) -> OptionNode<T> {
        Some(Rc::new(RefCell::new(AVLTreeNode {
            data,
            _left: None,
            _right: None,
            _height: 1,
        })))
    }

    fn _get_height(node: Option<RcRefcellAVLNode<T>>) -> u32 {
        node.map_or(0, |this_node| this_node.borrow()._height)
    }

    fn _get_left_height(node: &RcRefcellAVLNode<T>) -> u32 {
        Self::_get_height(node.borrow()._left.clone())
    }

    fn _get_right_height(node: &RcRefcellAVLNode<T>) -> u32 {
        Self::_get_height(node.borrow()._right.clone())
    }

    fn _get_balance_factor(node: &RcRefcellAVLNode<T>) -> i64 {
        Self::_get_left_height(node) as i64 - Self::_get_right_height(node) as i64
    }

    fn _left_rotate(root: RcRefcellAVLNode<T>) -> RcRefcellAVLNode<T> {
        let new_root: RcRefcellAVLNode<T> = root.borrow()._right.clone().unwrap();
        root.borrow_mut()._right = new_root.borrow()._left.clone().take();
        root.borrow_mut()._height = max(
            Self::_get_left_height(&root),
            Self::_get_right_height(&root),
        ) + 1;
        new_root.borrow_mut()._left = Some(root);
        new_root.borrow_mut()._height = max(
            Self::_get_left_height(&new_root),
            Self::_get_right_height(&new_root),
        ) + 1;
        new_root
    }

    fn _right_rotate(root: RcRefcellAVLNode<T>) -> RcRefcellAVLNode<T> {
        let new_root: RcRefcellAVLNode<T> = root.borrow()._left.clone().unwrap();
        root.borrow_mut()._left = new_root.borrow()._right.clone().take();
        root.borrow_mut()._height = max(
            Self::_get_left_height(&root),
            Self::_get_right_height(&root),
        ) + 1;
        new_root.borrow_mut()._right = Some(root);
        new_root.borrow_mut()._height = max(
            Self::_get_left_height(&new_root),
            Self::_get_right_height(&new_root),
        ) + 1;
        new_root
    }

    fn _left_right_rotate(root: RcRefcellAVLNode<T>) -> RcRefcellAVLNode<T> {
        let left: RcRefcellAVLNode<T> = root.borrow()._left.clone().take().unwrap();
        root.borrow_mut()._left = Some(Self::_left_rotate(left));
        Self::_right_rotate(root)
    }

    fn _right_left_rotate(root: RcRefcellAVLNode<T>) -> RcRefcellAVLNode<T> {
        let right: RcRefcellAVLNode<T> = root.borrow()._right.clone().take().unwrap();
        root.borrow_mut()._right = Some(Self::_right_rotate(right));
        Self::_left_rotate(root)
    }

    fn insert(node: OptionNode<T>, data: T) -> OptionNode<T> {
        let return_node: RcRefcellAVLNode<T> = match node {
            None => AVLTreeNode::new(data).unwrap(),
            Some(this_node) => {
                let node_data: T = this_node.borrow().data;
                match data.cmp(&node_data) {
                    Ordering::Less => {
                        let left: OptionNode<T> = this_node.borrow()._left.clone();
                        this_node.borrow_mut()._left = Self::insert(left, data);
                    }
                    Ordering::Greater => {
                        let right: OptionNode<T> = this_node.borrow()._right.clone();
                        this_node.borrow_mut()._right = Self::insert(right, data);
                    }
                    Ordering::Equal => {}
                }
                this_node
            }
        };
        let balance_factor: i64 = Self::_get_balance_factor(&return_node);
        let new_return_node: RcRefcellAVLNode<T> = match balance_factor {
            2 => {
                let new_data: T = return_node.borrow()._left.clone().unwrap().borrow().data;
                match data.cmp(&new_data) {
                    Ordering::Less => Self::_right_rotate(return_node),
                    Ordering::Greater => Self::_left_right_rotate(return_node),
                    _ => return_node,
                }
            }
            -2 => {
                let new_data: T = return_node.borrow()._right.clone().unwrap().borrow().data;
                match data.cmp(&new_data) {
                    Ordering::Less => Self::_right_left_rotate(return_node),
                    Ordering::Greater => Self::_left_rotate(return_node),
                    _ => return_node,
                }
            }
            _ => return_node,
        };
        new_return_node.borrow_mut()._height = max(
            Self::_get_left_height(&new_return_node),
            Self::_get_right_height(&new_return_node),
        ) + 1;
        Some(new_return_node)
    }

    fn delete(node: OptionNode<T>, data: T) -> OptionNode<T> {
        let return_node: OptionNode<T> = match node {
            None => {
                node
            }
            Some(this_node) => {
                let node_data: T = this_node.borrow().data;
                match node_data.cmp(&data) {
                    Ordering::Greater => {
                        let left: OptionNode<T> = this_node.borrow()._left.clone();
                        match left {
                            None => return Some(this_node),
                            Some(_) => {
                                let left: OptionNode<T> = this_node.borrow()._left.clone().take();
                                this_node.borrow_mut()._left = Self::delete(left, data);
                            }
                        }
                        Some(this_node)
                    }
                    Ordering::Less => {
                        let right: OptionNode<T> = this_node.borrow()._right.clone();
                        match right {
                            None => return Some(this_node),
                            Some(_) => {
                                let right: OptionNode<T> = this_node.borrow()._right.clone().take();
                                this_node.borrow_mut()._right = Self::delete(right, data);
                            }
                        }
                        Some(this_node)
                    }
                    Ordering::Equal => {
                        let left: OptionNode<T> = this_node.borrow()._left.clone();
                        let right: OptionNode<T> = this_node.borrow()._right.clone();
                        match (left.clone(), right.clone()) {
                            (Some(_), Some(inner_right)) => {
                                let min_value: T = inner_right.borrow().get_min();
                                this_node.borrow_mut().data = min_value;
                                let right: OptionNode<T> = this_node.borrow()._right.clone().take();
                                this_node.borrow_mut()._right = Self::delete(right, min_value);
                                Some(this_node)
                            }
                            (Some(inner_left), _) => Some(inner_left),
                            (_, Some(inner_right)) => Some(inner_right),
                            (_, _) => None,
                        }
                    }
                }
            }
        };
        match return_node {
            None => {
                return_node
            }
            Some(this_node) => {
                let balance_factor: i64 = Self::_get_balance_factor(&this_node);
                let return_node: RcRefcellAVLNode<T> = match balance_factor {
                    2 => {
                        let left_child: &RcRefcellAVLNode<T> = &this_node.borrow()._left.clone().unwrap();
                        let left_child_height: u32 = Self::_get_left_height(left_child);
                        let right_child_height: u32 = Self::_get_right_height(left_child);
                        match left_child_height.cmp(&right_child_height) {
                            Ordering::Greater | Ordering::Equal => Self::_right_rotate(this_node),
                            Ordering::Less => Self::_left_right_rotate(this_node)
                        }
                    }
                    -2 => {
                        let right_child: &RcRefcellAVLNode<T> = &this_node.borrow()._right.clone().unwrap();
                        let left_child_height: u32 = Self::_get_left_height(right_child);
                        let right_child_height: u32 = Self::_get_right_height(right_child);
                        match right_child_height.cmp(&left_child_height) {
                            Ordering::Greater | Ordering::Equal => Self::_left_rotate(this_node),
                            Ordering::Less => Self::_right_left_rotate(this_node)
                        }
                    }
                    _ => this_node,
                };
                return_node.borrow_mut()._height = max(
                    Self::_get_left_height(&return_node),
                    Self::_get_right_height(&return_node),
                ) + 1;
                Some(return_node)
            }
        }
    }

    fn print_node(&self, prefix_space: &String, child_prefix: String, is_right: bool) {
        match prefix_space.len() {
            6 => println!("|____ {} {:?}", child_prefix, self.data),
            _ => println!("{}{} {:?}", prefix_space, child_prefix, self.data),
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

#[derive(Debug)]
pub struct AVLTree<T: Ord + Copy + Debug> {
    _root: OptionNode<T>,
}

impl<T: Ord + Copy + Debug> Tree<T, AVLTreeNode<T>> for AVLTree<T> {
    /// Return the root node of the RBTree.
    ///
    /// # Examples
    /// ```
    /// use trees::avltree::AVLTree;
    /// use crate::trees::base::Tree;
    ///
    /// let mut tree = AVLTree::new();
    ///     let v=vec![1,2,3,4,5,6,7];
    /// for i in v {
    ///     tree.insert(i);
    /// }
    ///
    /// // Root 4
    /// // |____ L 2
    /// // |     |____ L 1
    /// // |     |____ R 3
    /// // |____ R 6
    /// //       |____ L 5
    /// //       |____ R 7
    /// let node = tree.get_root();
    /// println!("The value of root is {}", node.as_ref().unwrap().borrow().data);
    /// assert_eq!(node.as_ref().unwrap().borrow().data, 4);
    /// ```
    fn get_root(&self) -> &OptionNode<T> {
        &self._root
    }

    /// Inserting a new element.
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::avltree::AVLTree;
    /// use crate::trees::base::Tree;
    ///
    /// let mut tree = AVLTree::new();
    ///
    /// tree.insert(1);
    /// tree.insert(2);
    /// tree.insert(3);
    /// tree.insert(4);
    ///
    /// assert_eq!(tree.count_nodes(), 4);
    /// ```
    fn insert(&mut self, data: T) {
        if self.contain(data) == true {
            println!("This node already exists in the tree!");
        } else {
            match self._root.take() {
                Some(root) => self._root = AVLTreeNode::insert(Some(root), data),
                None => self._root = AVLTreeNode::new(data),
            }
        }
    }

    /// Remove the element with the target value.
    ///
    /// If target is missing from the tree, print "This node does not exist in the tree!"
    ///
    /// # Examples
    ///
    /// ```
    /// use trees::avltree::AVLTree;
    /// use crate::trees::base::Tree;
    ///
    /// let mut tree = AVLTree::new();
    ///     let v = vec![1, 2, 3, 4, 5, 6, 7];
    ///        for i in v {
    ///          tree.insert(i);
    ///          }
    ///
    /// assert_eq!(tree.count_nodes(), 7);
    /// tree.delete(7);
    /// assert_eq!(tree.count_nodes(), 6);
    ///
    /// // If you try to delete a value that is missing from the tree, nothing will change
    /// assert_eq!(tree.count_nodes(), 6);
    /// tree.delete(99);
    /// assert_eq!(tree.count_nodes(), 6);
    /// ```
    fn delete(&mut self, data: T) {
        if self.contain(data) == false {
            println!("This node does not exist in the tree!");
        } else {
            match self._root.take() {
                Some(root) => self._root = AVLTreeNode::delete(Some(root), data),
                None => return,
            }
        }
    }

    /// Print the AVLTree.
    ///
    /// # Examples
    /// ```
    /// use trees::avltree::AVLTree;
    /// use crate::trees::base::Tree;
    ///
    /// let mut tree = AVLTree::new();
    /// let v = vec![1, 2, 3, 4, 5, 6, 7];
    /// for i in v {
    ///    tree.insert(i);
    /// }
    /// // Root 4
    /// // |____ L 2
    /// // |     |____ L 1
    /// // |     |____ R 3
    /// // |____ R 6
    /// //       |____ L 5
    /// //       |____ R 7
    /// ```
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


impl<T: Ord + Copy + Debug> AVLTree<T> {
    pub fn new() -> Self {
        Self {
            _root: None
        }
    }

    /// Clear the AVLTree, removing all elements.
    ///
    /// # Examples
    /// ```
    /// use trees::avltree::AVLTree;
    /// use crate::trees::base::Tree;
    ///
    /// let mut tree = AVLTree::new();
    /// tree.insert(1);
    /// tree.clear();
    /// assert!(tree.is_empty());
    /// ```
    pub fn clear(&mut self) {
        *self = AVLTree::new();
        println!("Clear operation is complete!");
    }
}
