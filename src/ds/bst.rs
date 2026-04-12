use crate::ds::traits::BinarySearchTree;

struct Node<T> {
    value: T,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}
pub struct SimpleBST<T> {
    root: Option<Box<Node<T>>>,
}
impl<T: Ord> SimpleBST<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
}
impl<T: Ord> BinarySearchTree for SimpleBST<T> {
    type Item = T;
    fn volume(&self) -> usize {
        fn count_rec<T>(node: &Option<Box<Node<T>>>) -> usize {
            match node {
                Some(n) => 1 + count_rec(&n.left) + count_rec(&n.right),
                None => 0,
            }
        }
        count_rec(&self.root)
    }
    fn is_empty(&self) -> bool {
        self.volume() == 0
    }
    fn insert(&mut self, value: Self::Item) {
        fn insert_rec<T: Ord>(node: &mut Option<Box<Node<T>>>, value: T) {
            match node {
                Some(n) if value < n.value => insert_rec(&mut n.left, value),
                Some(n) if value > n.value => insert_rec(&mut n.right, value),
                Some(_) => {} //skip repeated value
                None => {
                    *node = Some(Box::new(Node {
                        value: value,
                        left: None,
                        right: None,
                    }));
                }
            }
        }
        insert_rec(&mut self.root, value);
    }
    fn contains(&self, value: &Self::Item) -> bool {
        fn search_rec<T: Ord>(node: &Option<Box<Node<T>>>, value: &T) -> bool {
            match node {
                Some(n) if value == &n.value => true,
                Some(n) if value < &n.value => search_rec(&n.left, value),
                Some(n) if value > &n.value => search_rec(&n.right, value),
                Some(_) => false,
                None => false,
            }
        }
        search_rec(&self.root, value)
    }
    fn remove(&mut self, value: &Self::Item) -> bool {
        fn get_successor<T: Ord>(node: &mut Option<Box<Node<T>>>) -> Option<T> {
            let mut curr = node.take()?;
            if curr.left.is_none() {
                *node = curr.right;
                Some(curr.value)
            } else {
                let min_val = get_successor(&mut curr.left);
                *node = Some(curr);
                min_val
            }
        }
        fn pop_successor<T: Ord>(
            node: &mut Option<Box<Node<T>>>,
            mut n: Box<Node<T>>,
            right: Box<Node<T>>,
        ) {
            let mut right_subtree = Some(right);
            if let Some(succ_val) = get_successor(&mut right_subtree) {
                n.value = succ_val;
                n.right = right_subtree;
                *node = Some(n);
            }
        }
        fn remove_curr_value<T: Ord>(node: &mut Option<Box<Node<T>>>) {
            let mut n = node.take().unwrap();
            let left = n.left.take();
            let right = n.right.take();

            match (left, right) {
                (None, None) => {}
                (Some(l), None) => *node = Some(l),
                (None, Some(r)) => *node = Some(r),
                (Some(_), Some(r)) => pop_successor(node, n, r),
            }
        }
        fn remove_rec<T: Ord>(node: &mut Option<Box<Node<T>>>, value: &T) -> bool {
            match node {
                Some(n) if value < &n.value => remove_rec(&mut n.left, value),
                Some(n) if value > &n.value => remove_rec(&mut n.right, value),
                Some(_) => {
                    remove_curr_value(node);
                    true
                }
                None => false,
            }
        }
        remove_rec(&mut self.root, value)
    }
}
