#[cfg(test)]
mod test {
    use crate::base::Tree;
    use crate::bstree;

    #[test]
    fn test_avl() {
        let mut bs_tree: bstree::BSTree<i32> = bstree::BSTree::new();
        assert_eq!(bs_tree.get_height(), 0);
        assert_eq!(bs_tree.is_empty(), true);
        assert_eq!(bs_tree.count_nodes(), 0);
        for number in vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
            bs_tree.insert(number);
        }
        assert_eq!(bs_tree.count_nodes(), 10);
        assert_eq!(bs_tree.get_min().unwrap(), 0);
        assert_eq!(bs_tree.get_max().unwrap(), 9);
        assert_eq!(bs_tree.is_empty(), false);
        assert_eq!(bs_tree.get_height(), 10);
        assert_eq!(bs_tree.count_leaves(), 1);
        for number in vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
            assert_eq!(bs_tree.contain(number), true);
        }
        for number in vec![0, 1, 2, 3, 4] {
            bs_tree.delete(number);
        }
        assert_eq!(bs_tree.count_nodes(), 5);
        assert_eq!(bs_tree.get_min().unwrap(), 5);
        assert_eq!(bs_tree.get_max().unwrap(), 9);
        assert_eq!(bs_tree.is_empty(), false);
        assert_eq!(bs_tree.get_height(), 5);
        assert_eq!(bs_tree.count_leaves(), 1);
        for number in vec![0, 1, 2, 3, 4] {
            assert_eq!(bs_tree.contain(number), false);
        }
        for number in vec![5, 6, 7, 8, 9] {
            assert_eq!(bs_tree.contain(number), true);
        }
        for number in vec![5, 6, 7, 8, 9] {
            bs_tree.delete(number);
        }
        assert_eq!(bs_tree.is_empty(), true);
    }
}