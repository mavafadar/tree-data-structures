mod avltree;
mod rbtree;
mod bstree;
mod base;

use crate::base::{TreeNode, Tree};
use rbtree::RBTree;
use rbtree::RBTreeNode;

fn main() {
    let mut tree = RBTree::new();
    let v=vec![8, 18, 5, 15, 17];
    for i in v {
        tree.insert(i);
    }
    let node = tree.get_root();
    println!("The value of root is {}",node.as_ref().unwrap().borrow().key);
    // tree.delete(9);
    //println!("The number of leaves: {}", tree.count_leaves());
    //println!("The height of tree: {}", tree.get_height());
    //println!("The tree is empty? {}", tree.is_empty());
    //tree.traverse_postorder();
    //println!();
    //tree.print_tree();
    //tree.clear();
    //println!("The number of leaves: {}", tree.count_leaves());
}
