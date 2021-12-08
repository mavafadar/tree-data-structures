// mod avltree;
//mod rbtree;
// mod bstree;
//mod base;
mod cli;


// use crate::base::{TreeNode, Tree};
// use rbtree::RBTree;
// use rbtree::RBTreeNode;
// use std::env;


fn main() {
    cli::welcome();
    cli::start_cli();
    
    // let mut tree = RBTree::new();
    // let v=vec![1, 2, 3, 4, 5, 6, 7];
    // for i in v {
    //     tree.insert(i);
    // }
    // tree.delete(5);
    //let node = tree.get_root();
    //println!("The value of root is {}",node.as_ref().unwrap().borrow().key);
    // tree.delete(9);
    //println!("The number of leaves: {}", tree.count_leaves());
    //println!("The height of tree: {}", tree.get_height());
    //println!("The tree is empty? {}", tree.is_empty());
    //tree.traverse_preorder();
    //println!();
    //tree.print_tree();
    //tree.clear();
    //println!("The number of leaves: {}", tree.count_leaves());
}
