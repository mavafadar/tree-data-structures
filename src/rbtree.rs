use std::cell::{RefCell, Ref, RefMut};
use std::rc::Rc;
use std::fmt::Debug;

use crate::base::{TreeNode, Tree};

#[derive(Clone, Debug, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

//type Tree<T>= Rc<RefCell<TreeNode<T>>>;
//type RedBlackTree<T>= Option<Tree<T>>;
// In order to reuse the code, we change the struct name
type RcRefcellRBTNode<T> = Rc<RefCell<RBTreeNode<T>>>;
type OptionNode<T> = Option<RcRefcellRBTNode<T>>;

/// RBTreeNode is a node in the RBTree.
/// key is the value of the node.
/// color is the color of the node, black or red.
#[derive(Debug)]
pub struct RBTreeNode<T: Ord + Copy + Debug> {
    pub key: T,
    color: NodeColor,
    parent: OptionNode<T>,
    left: OptionNode<T>,
    right: OptionNode<T>,
}

/// A red black tree is a kind of self-balancing binary search tree
/// that can be used to store elements.
/// The root node is the root node of the red black tree.
#[derive(Debug)]
pub struct RBTree<T: Ord + Copy + Debug> {
    root: OptionNode<T>,
}

impl<T: Ord + Copy + Debug> TreeNode<T> for RBTreeNode<T> {
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

impl<T: Ord + Copy + Debug> RBTreeNode<T> {
    fn new(value: T) -> RBTreeNode<T> {
        RBTreeNode {
            color: NodeColor::Red,
            key: value,
            parent: None,
            left: None,
            right: None,
        }
    }

    fn print_node(&self, prefix_space: &String, child_prefix: String, is_right: bool) {
        let color = if self.color == NodeColor::Black {
            "Black"
        } else {
            "Red"
        };
        match prefix_space.len() {
            6 => println!("|____ {} {:?} {}", child_prefix, self.key, color),
            _ => println!("{}{} {:?} {}", prefix_space, child_prefix, self.key, color),
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


impl<T: Ord + Copy + Debug> RBTree<T> {
    /// Makes a new empty RBTree.
    ///
    /// Does not allocate anything on its own.
    pub fn new() -> Self {
        RBTree {
            root: None,
        }
    }

    fn _insert_repair(&mut self, new_child: RcRefcellRBTNode<T>) {
        let mut child: RcRefcellRBTNode<T> = new_child;
        loop {
            // child = new_child;
            if child.borrow().parent.is_none() {
                // 1. no root
                Self::_change_color(&mut self.root.as_ref().unwrap(), NodeColor::Black);
                return ;
            }
            if !Self::_is_parent_red(&child) {
                // 2. parent black do nothing
                return ;
            }
            // 3.parent is red
            let mut parent: Rc<RefCell<RBTreeNode<T>>> = Rc::clone(child.borrow().parent.as_ref().unwrap());
            // grandparent exists
            let grandparent: Rc<RefCell<RBTreeNode<T>>> = Rc::clone(parent.borrow().parent.as_ref().unwrap());
            let uncle: Rc<RefCell<RBTreeNode<T>>>;

            // find uncle node
            let parent_left_side: bool = RBTree::_is_left_child(&parent);
            if parent_left_side {
                //3.2 uncle is right node. uncle black or is_none
                if grandparent.borrow().right.is_none()
                    || grandparent.borrow().right.as_ref().unwrap().borrow().color == NodeColor::Black
                {
                    if !RBTree::_is_left_child(&child) {
                        //3.2.2 child is right node
                        self._left_rotate(&parent);
                        parent = grandparent.borrow().left.as_ref().unwrap().clone();
                        // child = parent;
                    }
                    //3.2.1 child is left node
                    RBTree::_change_color(&mut &parent, NodeColor::Black);
                    RBTree::_change_color(&mut &grandparent, NodeColor::Red);
                    self._right_rotate(&grandparent);
                    // grandparent.borrow_mut().rotate_right();
                    return;
                } else {
                    //3.2.3
                    // if right side uncle exists
                    uncle = grandparent.borrow().right.as_ref().unwrap().clone();
                    RBTree::_change_color(&mut &parent, NodeColor::Black);
                    RBTree::_change_color(&mut &uncle, NodeColor::Black);
                    RBTree::_change_color(&mut &grandparent, NodeColor::Red);
                    // We've solved the problem at our node, but grandparent may have the same issue, so run it again.
                    child = grandparent;
                    continue;
                }
            } else {
                //3.3 uncle is left node. uncle black or is_none
                if grandparent.borrow().left.is_none()
                    || grandparent.borrow().left.as_ref().unwrap().borrow().color
                    == NodeColor::Black
                {
                    if RBTree::_is_left_child(&child) {
                        //3.3.2 child is left child
                        self._right_rotate(&parent);
                        parent = grandparent.borrow().right.as_ref().unwrap().clone();
                    }
                    //3.3.1 child is right child
                    RBTree::_change_color(&mut &parent, NodeColor::Black);
                    RBTree::_change_color(&mut &grandparent, NodeColor::Red);
                    self._left_rotate(&grandparent);

                    // grandparent.borrow_mut().rotate_left();
                    return ;
                } else {
                    //3.3.3
                    // if left side uncle exists
                    uncle = grandparent.borrow().left.as_ref().unwrap().clone();
                    RBTree::_change_color(&mut &parent, NodeColor::Black);
                    RBTree::_change_color(&mut &uncle, NodeColor::Black);
                    RBTree::_change_color(&mut &grandparent, NodeColor::Red);
                    // We've solved the problem at our node, but grandparent may have the same issue, so run it again.
                    child = grandparent;
                    continue;
                }
            }
        }
    }

    fn search(&mut self, value: T) -> (bool, OptionNode<T>) {
        let mut parent = None;
        if self.root.is_none() {
            return (false, None);
        }
        let mut child = Some(self.root.as_ref().unwrap().clone());
        while child.is_some() {
            parent = child;
            let parent_node = parent.as_ref().unwrap();
            // get the key
            if parent_node.borrow().key > value {
                child = match parent_node.borrow().left {
                    Some(ref node) => Some(node.clone()),
                    None => None,
                };
            } else if parent_node.borrow().key < value {
                child = match parent_node.borrow().right {
                    Some(ref node) => Some(node.clone()),
                    None => None,
                };
            } else { return (true, parent); }
        }
        return (false, parent);
    }

    fn _is_left_child(node: &RcRefcellRBTNode<T>) -> bool {
        let child_node: Ref<RBTreeNode<T>> = node.borrow();
        let parent: &&RcRefcellRBTNode<T> = &child_node.parent.as_ref().unwrap();
        let parent_node: Ref<RBTreeNode<T>> = parent.borrow();
        match parent_node.left.as_ref() {
            Some(x) => x.borrow().key == child_node.key,
            None => false,
        }
    }

    fn _is_parent_red(node: &RcRefcellRBTNode<T>) -> bool {
        match &node.borrow().parent.as_ref() {
            None => false,
            Some(x) => x.borrow().color == NodeColor::Red,
        }
    }

    fn _left_rotate(&mut self, rotation_node: &RcRefcellRBTNode<T>) {
        // x_parent and r are Option
        {
            let parent: &OptionNode<T> = &rotation_node.borrow().parent;
            let right: &OptionNode<T> = &rotation_node.borrow().right;
            //adjust x.parent
            //whether x is root
            if rotation_node.borrow().parent.is_none() {
                self.root = right.clone();
            }
            if let Some(node) = parent {
                if Self::_is_left_child(rotation_node) {
                    node.borrow_mut().left = right.clone();
                } else {
                    node.borrow_mut().right = right.clone();
                }
            }
            right.as_ref().unwrap().borrow_mut().parent = parent.clone();
        }
        let right_node: Rc<RefCell<RBTreeNode<T>>> = rotation_node.borrow().right.as_ref().unwrap().clone();
        rotation_node.borrow_mut().parent = Some(Rc::clone(&right_node));
        //adjust rl and x be the left child of r
        if right_node.borrow().left.is_some() {
            rotation_node.borrow_mut().right = Some(right_node.borrow().left.as_ref().unwrap().clone());
            right_node.borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(Rc::clone(rotation_node));
        } else {
            rotation_node.borrow_mut().right = None;
        }
        right_node.borrow_mut().left = Some(rotation_node.clone());
    }

    fn _right_rotate(&mut self, rotation_node: &RcRefcellRBTNode<T>) {
        {
            let parent: &OptionNode<T> = &rotation_node.borrow().parent;
            let left: &OptionNode<T> = &rotation_node.borrow().left;
            if rotation_node.borrow().parent.is_none() {
                self.root = left.clone();
            }
            if let Some(node) = parent {
                if Self::_is_left_child(rotation_node) {
                    node.borrow_mut().left = left.clone();
                } else {
                    node.borrow_mut().right = left.clone();
                }
            }
            left.as_ref().unwrap().borrow_mut().parent = parent.clone();
        }
        let left_node: Rc<RefCell<RBTreeNode<T>>> = rotation_node.borrow().left.as_ref().unwrap().clone();
        rotation_node.borrow_mut().parent = Some(Rc::clone(&left_node));
        if left_node.borrow().right.is_some() {
            rotation_node.borrow_mut().left = Some(left_node.borrow().right.as_ref().unwrap().clone());
            left_node.borrow_mut().right.as_ref().unwrap().borrow_mut().parent = Some(Rc::clone(rotation_node));
        } else {
            rotation_node.borrow_mut().left = None;
        }
        left_node.borrow_mut().right = Some(rotation_node.clone());
    }

    // recursive find the left child
    fn _recur_right_child(node: OptionNode<T>) -> OptionNode<T> {
        if node.as_ref().unwrap().borrow().right.is_some() {
            return Self::_recur_right_child(node.as_ref().unwrap().borrow().right.clone());
        }
        return node;
    }

    // find the replacement node to replace the delete node 
    /// Inorder predecessor of the element with the specified value
    /// 
    /// In Red-black Tree, inorder predecessor of an input node can be defined as 
    /// the node with the greatest value smaller than the value of the input node.
    /// 
    fn _find_replacement_node(node: &RcRefcellRBTNode<T>) -> OptionNode<T> {
        return if node.borrow().left.is_some() && node.borrow().right.is_some() {
            Self::_recur_right_child(node.borrow().left.clone())
        } else if node.borrow().left.is_some() {
            node.borrow().left.clone()
        } else if node.borrow().right.is_some() {
            node.borrow().right.clone()
        } else {
            None
        }
    }

    fn _delete_private(&mut self, node: &mut &RcRefcellRBTNode<T>) -> Result<(), String> {
        // replacement Node
        let replacement: OptionNode<T> = Self::_find_replacement_node(node);
        // parent Node of node
        let parent: Option<Rc<RefCell<RBTreeNode<T>>>> = if node.borrow().parent.is_some() {
            Some(Rc::clone(node.borrow().parent.as_ref().unwrap()))
        } else {
            None
        };
        let double_black: bool = Self::_return_color(node) == NodeColor::Black
            && (replacement.is_none()
            || Self::_return_color(replacement.as_ref().unwrap()) == NodeColor::Black);

        // 1. node is Leaf Node
        if replacement.is_none() {
            // node is root
            if node.borrow().parent.is_none() {
                self.root = None;
            } else {
                if double_black {
                    self._delete_repair(node);
                }
                // delete node
                if Self::_is_left_child(node) {
                    parent.as_ref().unwrap().borrow_mut().left = None;
                } else {
                    parent.as_ref().unwrap().borrow_mut().right = None;
                }
            }
            return Ok(());
        }
        // 2. node only has one child
        else if node.borrow().left.is_none() || node.borrow().right.is_none() {
            // node is root, the tree only has two nodes.
            if node.borrow().parent.is_none() {
                let temp: T = replacement.as_ref().unwrap().borrow().key;
                let mut root: RefMut<RBTreeNode<T>> = self.root.as_ref().unwrap().borrow_mut();
                root.key = temp;
                root.left = None;
                root.right = None;
            } else {
                // set parent's child
                if !Self::_is_left_child(node) {
                    parent.as_ref().unwrap().borrow_mut().right = replacement.clone();
                } else {
                    parent.as_ref().unwrap().borrow_mut().left = replacement.clone();
                }
                // set replacement's parent
                replacement.as_ref().unwrap().borrow_mut().parent = parent.clone();
                // doubled black needs adjust, one red just set R black. impossible double red
                if !double_black {
                    Self::_change_color(&mut replacement.as_ref().unwrap(), NodeColor::Black);
                } else {
                    self._delete_repair(&replacement.unwrap());
                }
            }
            return Ok(());
        }
        // 3. node has two children
        else {
            // actually delete replacement.
            let temp: T = replacement.as_ref().unwrap().borrow().key;
            node.borrow_mut().key = temp;
            self._delete_private(&mut replacement.as_ref().unwrap()).unwrap();
        }
        Ok(())
    }

    fn _delete_repair(&mut self, node: &RcRefcellRBTNode<T>) {
        if node.borrow().parent.is_none() {
            return;
        }
        //get parent Node
        let parent = Some(Rc::clone(node.borrow().parent.as_ref().unwrap()));
        // get sibling Node
        let sibling: OptionNode<T> = Self::_return_node_same_level(node);

        //1.no sibling, adjust parent
        if sibling.is_none() {
            self._delete_repair(&parent.unwrap())
        } else {
            // 2.sibling is black
            if Self::_return_color(sibling.as_ref().unwrap()) == NodeColor::Black {
                // 2.1 sibling doesnt have red child
                if !Self::_has_red_child(sibling.as_ref().unwrap()) {
                    Self::_change_color(&mut sibling.as_ref().unwrap(), NodeColor::Red);
                    if Self::_return_color(parent.as_ref().unwrap()) == NodeColor::Red {
                        Self::_change_color(&mut parent.as_ref().unwrap(), NodeColor::Black);
                    } else {
                        self._delete_repair(&parent.unwrap());
                    }
                } else {
                    // 2.2.1 ll
                    if !Self::_is_left_child(node) {
                        if sibling.as_ref().unwrap().borrow().left.is_some() && Self::_return_color(sibling.as_ref().unwrap().borrow().left.as_ref().unwrap()) == NodeColor::Red {
                            Self::_change_color(&mut sibling.as_ref().unwrap().borrow().left.as_ref().unwrap(), NodeColor::Black);
                            let parent_color = Self::_return_color(parent.as_ref().unwrap());
                            Self::_change_color(&mut sibling.as_ref().unwrap(), parent_color);
                            self._right_rotate(parent.as_ref().unwrap());
                            Self::_change_color(&mut parent.as_ref().unwrap(), NodeColor::Black);
                        } else {
                            // 2.2.2 lr
                            let parent_color = Self::_return_color(parent.as_ref().unwrap());
                            Self::_change_color(&mut sibling.as_ref().unwrap().borrow().right.as_ref().unwrap(), parent_color);
                            self._left_rotate(sibling.as_ref().unwrap());
                            self._right_rotate(parent.as_ref().unwrap());
                            Self::_change_color(&mut parent.as_ref().unwrap(), NodeColor::Black);
                        }
                    } else {
                        // 2.2.3 rl
                        if sibling.as_ref().unwrap().borrow().left.is_some() && Self::_return_color(sibling.as_ref().unwrap().borrow().left.as_ref().unwrap()) == NodeColor::Red {
                            let parent_color: NodeColor = Self::_return_color(parent.as_ref().unwrap());
                            Self::_change_color(&mut sibling.as_ref().unwrap().borrow().left.as_ref().unwrap(), parent_color);
                            self._right_rotate(sibling.as_ref().unwrap());
                            self._left_rotate(parent.as_ref().unwrap());
                            Self::_change_color(&mut parent.as_ref().unwrap(), NodeColor::Black);
                        } else {
                            // 2.2.4 rr
                            Self::_change_color(&mut sibling.as_ref().unwrap().borrow().right.as_ref().unwrap(), NodeColor::Black);
                            let parent_color: NodeColor = Self::_return_color(parent.as_ref().unwrap());
                            Self::_change_color(&mut sibling.as_ref().unwrap(), parent_color);
                            self._left_rotate(parent.as_ref().unwrap());
                            Self::_change_color(&mut parent.as_ref().unwrap(), NodeColor::Black);
                        }
                    }
                }
            }
            // 3.sibling is red
            else {
                Self::_change_color(&mut sibling.as_ref().unwrap(), NodeColor::Black);
                Self::_change_color(&mut parent.as_ref().unwrap(), NodeColor::Red);
                // if R is left_child. left_rotate
                if Self::_is_left_child(node) {
                    self._left_rotate(parent.as_ref().unwrap());
                } else {
                    self._right_rotate(parent.as_ref().unwrap());
                }
                self._delete_repair(node);
            }
        }
    }

    fn _return_color(node: &RcRefcellRBTNode<T>) -> NodeColor {
        node.borrow().color.clone()
    }

    fn _change_color(node: &mut &RcRefcellRBTNode<T>, color: NodeColor) {
        node.borrow_mut().color = color;
    }

    fn _return_node_same_level(node:&RcRefcellRBTNode<T>)->OptionNode<T> {
        let borrowed_node=node.borrow();
        if borrowed_node.parent.is_some() {
            let parent_node=&borrowed_node.parent.as_ref().unwrap().borrow();
            if Self::_is_left_child(node) {
                return parent_node.right.clone()
            }else {
                return parent_node.left.clone()
            }
        }
        return None
    }

    fn _has_red_child(node: &RcRefcellRBTNode<T>) -> bool {
        let condition_one: bool = node.borrow().left.is_some() && Self::_return_color(node.borrow().left.as_ref().unwrap()) == NodeColor::Red;
        let condition_two: bool = node.borrow().right.is_some() && Self::_return_color(node.borrow().right.as_ref().unwrap()) == NodeColor::Red;
        condition_one || condition_two
    }

    /// Clear the RBTree, removing all elements.
    ///
    /// # Examples
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree = RBTree::new();
    /// tree.insert(1);
    /// tree.clear();
    /// assert!(tree.is_empty());
    /// ```
    pub fn clear(&mut self) {
        *self = RBTree::new();
        println!("Clear operation is complete!");
    }
}

impl<T: Ord + Copy + Debug> Tree<T, RBTreeNode<T>> for RBTree<T> {
    /// Return the root node of the RBTree.
    ///
    /// # Examples
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree = RBTree::new();
    ///     let v=vec![1,2,3,4,5,6,7];
    /// for i in v {
    ///     tree.insert(i);
    /// }
    /// 
    /// // The tree is like this.
    /// // Root 2 Black
    /// // |____ L 1 Black
    /// // |____ R 4 Red
    /// //       |____ L 3 Black
    /// //       |____ R 6 Black
    /// //             |____ L 5 Red
    /// //             |____ R 7 Red
    /// let node = tree.get_root();
    /// println!("The value of root is {}",node.as_ref().unwrap().borrow().key);
    /// assert_eq!(node.as_ref().unwrap().borrow().key, 2);
    /// ```
    fn get_root(&self) -> &OptionNode<T> {
        &self.root
    }

    /// Inserting a new element.
    ///
    /// # Examples
    /// 
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree = RBTree::new();
    /// 
    /// tree.insert(1);
    /// tree.insert(2);
    /// tree.insert(3);
    /// tree.insert(4);
    /// 
    /// assert_eq!(tree.count_nodes(), 4);
    /// ```
    fn insert(&mut self, value: T) {
        match self.is_empty() {
            true => { // 1. tree is empty
                let mut new_node = RBTreeNode::new(value);
                new_node.color = NodeColor::Black;
                self.root = Some(Rc::new(RefCell::new(new_node)));
            }
            false => {
                let (found, parent_option) = self.search(value);
                match found {
                    true => (println!("The node already exists in the tree.")), // 2. node already exists
                    false => {
                        // 3. insert node
                        let child_belongs_on_left: bool = value < parent_option.as_ref().unwrap().borrow().key;
                        let new_child_node: RcRefcellRBTNode<T> = Rc::new(RefCell::new(RBTreeNode::new(value)));
                        let new_child_ref_clone: RcRefcellRBTNode<T> = new_child_node.clone();
                        let new_child = Some(new_child_node);
                        // set the new_child's parent
                        let parent_ref: RcRefcellRBTNode<T> = Rc::clone(&parent_option.as_ref().unwrap());
                        new_child.as_ref().unwrap().borrow_mut().parent = Some(parent_ref);
                        // put it on the side that it should be on
                        match child_belongs_on_left {
                            true => parent_option.as_ref().unwrap().borrow_mut().left = new_child,
                            false => parent_option.as_ref().unwrap().borrow_mut().right = new_child,
                        }
                        self._insert_repair(new_child_ref_clone);
                    }
                }
            }
        }
    }

    /// Remove the element with the target value.
    /// 
    /// If target is missing from the tree, print "The node of doesn't exist."
    /// 
    /// # Examples
    /// 
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree = RBTree::new();
    ///     let v=vec![1,2,3,4,5,6,7];
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
    fn delete(&mut self, value: T) {
        let (flag, searched_node) = self.search(value);
        match flag {
            false => {
                println!("The node of value {:#?} doesn't exist.",value);
            }
            true => {
                let mut searched_node_ref: &RcRefcellRBTNode<T> = searched_node.as_ref().unwrap();
                let _ = self._delete_private(&mut searched_node_ref);
            }
        };
    }

    /// Print the RBTree.
    ///
    /// # Examples
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree = RBTree::new();
    /// let v=vec![1,2,3,4,5,6,7];
    /// for i in v {
    ///    tree.insert(i);
    /// }
    /// // The result is like this.
    /// // Root 2 Black
    /// // |____ L 1 Black
    /// // |____ R 4 Red
    /// //       |____ L 3 Black
    /// //       |____ R 6 Black
    /// //             |____ L 5 Red
    /// //             |____ R 7 Red
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
