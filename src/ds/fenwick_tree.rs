use crate::ds::traits::IndexTree;
use std::ops::{Add, Sub};

pub struct FenwickSumTree<T> {
    tree: Vec<T>,
}

impl<T> FenwickSumTree<T>
where
    T: Copy + Default + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(n: usize) -> Self {
        Self {
            tree: vec![T::default(); n + 1],
        }
    }
}

impl<T> IndexTree for FenwickSumTree<T>
where
    T: Copy + Default + Add<Output = T> + Sub<Output = T>,
{
    type Item = T;

    fn len(&self) -> usize {
        self.tree.len()
    }

    fn add(&mut self, mut index: usize, delta: Self::Item) {
        while index < self.tree.len() {
            self.tree[index] = self.tree[index] + delta;
            index += index & index.wrapping_neg();
        }
    }
    fn prefix_query(&self, mut index: usize) -> T {
        let mut query = T::default();
        while index > 0 {
            query = query + self.tree[index];
            index -= index & index.wrapping_neg();
        }
        query
    }

    fn range_query(&self, left_idx: usize, right_idx: usize) -> T {
        self.prefix_query(right_idx) - self.prefix_query(left_idx - 1)
    }
}
