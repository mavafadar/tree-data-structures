use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use trees::{rbtree, bstree, avltree};
use trees::base::Tree;
use trees::rbtree::RBTree;
use trees::bstree::BSTree;
use trees::avltree::AVLTree;

fn bench_rbtree(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("rbtree_test");
    for size in [10000, 40000, 70000, 100000, 130000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |bench, &size| {
            bench.iter(|| {
                let mut tree: RBTree<i32> = rbtree::RBTree::new();
                for index in 1..size {
                    tree.insert(index);
                }
                for index in 0..size / 10 {
                    tree.contain(index);
                }
            })
        });
    }
    group.finish();
}

fn bench_bstree(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("bstree_test");
    for size in [10, 100, 1000, 5000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |bench, &size| {
            bench.iter(|| {
                let mut tree: BSTree<i32> = bstree::BSTree::new();
                for index in 1..size {
                    tree.insert(index);
                }
                for index in 0..size / 10 {
                    tree.contain(index);
                }
            })
        });
    }
    group.finish();
}

fn bench_avl_tree(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("avltree_test");
    for size in [10000, 40000, 70000, 100000, 130000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |bench, &size| {
            bench.iter(|| {
                let mut tree: AVLTree<i32> = avltree::AVLTree::new();
                for index in 1..size {
                    tree.insert(index);
                }
                for index in 0..size / 10 {
                    tree.contain(index);
                }
            })
        });
    }
    group.finish();
}

fn bench_rbtree_insertion(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("rbtree_test");
    for size in [10000, 40000, 70000, 100000, 130000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |bench, &size| {
            bench.iter(|| {
                let mut tree: RBTree<i32> = rbtree::RBTree::new();
                for index in 1..size {
                    tree.insert(index);
                }
            })
        });
    }
    group.finish();
}

fn bench_bstree_insertion(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("bstree_test");
    for size in [10, 100, 1000, 5000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |bench, &size| {
            bench.iter(|| {
                let mut tree: BSTree<i32> = bstree::BSTree::new();
                for index in 1..size {
                    tree.insert(index);
                }
            })
        });
    }
    group.finish();
}

fn bench_avl_tree_insertion(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("avltree_test");
    for size in [10000, 40000, 70000, 100000, 130000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |bench, &size| {
            bench.iter(|| {
                let mut tree: AVLTree<i32> = avltree::AVLTree::new();
                for index in 1..size {
                    tree.insert(index);
                }
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_rbtree_insertion, bench_bstree_insertion, bench_avl_tree_insertion);
criterion_main!(benches);