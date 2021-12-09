#[cfg(test)]
mod test {
    use crate::base::Tree;
    use crate::avltree;

    #[test]
    fn test_avl() {
        let mut avl_tree: avltree::AVLTree<i32> = avltree::AVLTree::new();
        assert_eq!(avl_tree.get_height(), 0);
        assert_eq!(avl_tree.is_empty(), true);
        assert_eq!(avl_tree.count_nodes(), 0);
        for number in vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
            avl_tree.insert(number);
        }
        assert_eq!(avl_tree.count_nodes(), 10);
        assert_eq!(avl_tree.get_min().unwrap(), 0);
        assert_eq!(avl_tree.get_max().unwrap(), 9);
        assert_eq!(avl_tree.is_empty(), false);
        assert_eq!(avl_tree.get_height(), 4);
        assert_eq!(avl_tree.count_leaves(), 5);
        for number in vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
            assert_eq!(avl_tree.contain(number), true);
        }
        for number in vec![0, 1, 2, 3, 4] {
            avl_tree.delete(number);
        }
        assert_eq!(avl_tree.count_nodes(), 5);
        assert_eq!(avl_tree.get_min().unwrap(), 5);
        assert_eq!(avl_tree.get_max().unwrap(), 9);
        assert_eq!(avl_tree.is_empty(), false);
        assert_eq!(avl_tree.get_height(), 3);
        assert_eq!(avl_tree.count_leaves(), 2);
        for number in vec![0, 1, 2, 3, 4] {
            assert_eq!(avl_tree.contain(number), false);
        }
        for number in vec![5, 6, 7, 8, 9] {
            assert_eq!(avl_tree.contain(number), true);
        }
        for number in vec![5, 6, 7, 8, 9] {
            avl_tree.delete(number);
        }
        assert_eq!(avl_tree.is_empty(), true);
    }
}