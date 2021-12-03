mod avltree;
mod rbtree;
mod base;

use crate::base::{TreeNode, Tree};

fn main() {
    let mut tree = rbtree::RBTree::new();
    tree.insert(1);
    tree.insert(2);
    tree.insert(3);
    tree.insert(4);
    tree.insert(5);
    tree.insert(6);
    tree.insert(7);
    tree.insert(8);
    tree.delete(8);
    tree.delete(5);
    println!("The number of leaves: {}", tree.count_leaves());
    println!("The height of tree: {}", tree.get_height());
    println!("The tree is empty? {}", tree.is_empty());
    tree.traverse_inorder();
    println!();
    tree.print_tree();
}
