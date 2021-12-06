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
            left.borrow().traverse_preorder();
        }
        if let Some(right) = self.get_right() {
            right.borrow().traverse_preorder();
        }
    }

    fn traverse_postorder(&self) {
        if let Some(left) = self.get_left() {
            left.borrow().traverse_postorder();
        }
        if let Some(right) = self.get_right() {
            right.borrow().traverse_postorder();
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

    /// The height of the tree.
    /// 
    /// This function can be used in RBTree, AVLTree and BSTree.
    /// 
    /// Returns to the height.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree = RBTree::new();
    /// assert_eq!(tree.get_height(), 0);
    /// 
    /// tree.insert(1); 
    /// tree.insert(0); 
    /// tree.insert(2); 
    /// assert_eq!(tree.get_height(), 2);
    /// ```
    fn get_height(&self) -> u32 {
        match &self.get_root() {
            None => 0,
            Some(node) => node.borrow().get_height(),
        }
    }

    /// The minimum element of the tree.
    /// 
    /// This function can be used in RBTree, AVLTree and BSTree.
    /// 
    /// Returns to the minimum element.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree = RBTree::new();
    /// assert_eq!(tree.get_min(), None);
    /// 
    /// tree.insert(1); 
    /// tree.insert(0); 
    /// tree.insert(2); 
    /// assert_eq!(tree.get_min(), Some(0));
    /// ```
    fn get_min(&self) -> Option<T> {
        match &self.get_root() {
            None => None,
            Some(node) => Some(node.borrow().get_min()),
        }
    }

    /// The maximum element of the tree.
    /// 
    /// This function can be used in RBTree, AVLTree and BSTree.
    /// 
    /// Returns to the maximum element.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree = RBTree::new();
    /// assert_eq!(tree.get_max(), None);
    /// 
    /// tree.insert(1); 
    /// tree.insert(0); 
    /// tree.insert(2); 
    /// assert_eq!(tree.get_max(), Some(2));
    /// ```
    fn get_max(&self) -> Option<T> {
        match &self.get_root() {
            None => None,
            Some(node) => Some(node.borrow().get_max()),
        }
    }

    /// Returns the number of leaf nodes in the tree.
    ///
    /// This function can be used in RBTree, AVLTree and BSTree.
    /// 
    /// # Examples
    ///
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree = RBTree::new();
    /// assert_eq!(tree.count_leaves(), 0);
    /// tree.insert(1);
    /// tree.insert(2);
    /// assert_eq!(tree.count_leaves(), 1);
    /// ```
    fn count_leaves(&self) -> u32 {
        match &self.get_root() {
            None => 0,
            Some(node) => node.borrow().count_leaves(),
        }
    }

    /// Returns the number of elements in the tree.
    ///
    /// This function can be used in RBTree, AVLTree and BSTree.
    /// 
    /// # Examples
    ///
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree = RBTree::new();
    /// assert_eq!(tree.count_nodes(), 0);
    /// tree.insert(1);
    /// assert_eq!(tree.count_nodes(), 1);
    /// ```
    fn count_nodes(&self) -> u32 {
        match &self.get_root() {
            None => 0,
            Some(node) => node.borrow().count_nodes(),
        }
    }

    /// Inorder traverse iterator of tree.
    /// 
    /// This function can be used in RBTree, AVLTree and BSTree.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree= RBTree::new();
    ///     let v=vec![1,2,3,4,5,6,7];
    ///        for i in v {
    ///            tree.insert(i);
    ///        }
    /// 
    /// // Now we have a tree that looks like this:
    /// //                  2
    /// //               1     4
    /// //                   3   6
    /// //                      5 7
    /// 
    /// // And we should get the following sequence of its elements: 1, 2, 3, 4, 5, 6, 7
    /// tree.traverse_inorder();
    /// ```
    fn traverse_inorder(&self) {
        match &self.get_root() {
            None => return ,
            Some(node) => node.borrow().traverse_inorder(),
        }
    }

    /// Preorder traverse iterator of tree.
    /// 
    /// This function can be used in RBTree, AVLTree and BSTree.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree= RBTree::new();
    ///     let v=vec![1,2,3,4,5,6,7];
    ///        for i in v {
    ///            tree.insert(i);
    ///        }
    /// 
    /// // Now we have a tree that looks like this:
    /// //                  2
    /// //               1     4
    /// //                   3   6
    /// //                      5 7
    /// 
    /// // And we should get the following sequence of its elements: 2, 1, 4, 3, 6, 5, 7
    /// tree.traverse_preorder();
    /// ```
    fn traverse_preorder(&self) {
        match &self.get_root() {
            None => return ,
            Some(node) => node.borrow().traverse_preorder(),
        }
    }

    /// Postorder traverse iterator of tree.
    /// 
    /// This function can be used in RBTree, AVLTree and BSTree.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree= RBTree::new();
    ///     let v=vec![1,2,3,4,5,6,7];
    ///        for i in v {
    ///            tree.insert(i);
    ///        }
    /// 
    /// // Now we have a tree that looks like this:
    /// //                  2
    /// //               1     4
    /// //                   3   6
    /// //                      5 7
    /// 
    /// // And we should get the following sequence of its elements: 1, 3, 5, 7, 6, 4, 2
    /// tree.traverse_postorder();
    /// ```
    fn traverse_postorder(&self) {
        match &self.get_root() {
            None => return ,
            Some(node) => node.borrow().traverse_postorder(),
        }
    }

    /// Checks whether the tree contains an element with the specified value.
    /// 
    /// This function can be used in RBTree, AVLTree and BSTree.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree = RBTree::new();
    /// assert_eq!(tree.contains(1), false);
    /// 
    /// tree.insert(1); 
    /// tree.insert(0); 
    /// tree.insert(2); 
    /// tree.insert(1);
    /// 
    /// assert!(tree.contains(2));
    /// assert!(tree.contains(1));
    /// assert!(!tree.contains(999));
    /// ```
    fn contains(&self, value: T) -> bool {
        match &self.get_root() {
            None => false,
            Some(node) => node.borrow().contains(value),
        }
    }

    /// Сhecking if the tree is empty.
    /// 
    /// This function can be used in RBTree, AVLTree and BSTree.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use trees::rbtree::RBTree;
    /// use crate::trees::base::Tree;
    /// 
    /// let mut tree = RBTree::new();
    /// assert!(tree.is_empty());
    /// 
    /// tree.insert(1);
    /// assert!(!tree.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        match &self.get_root() {
            Some(_) => false,
            None => true,
        }
    }

}
