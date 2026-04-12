use std::ops::{Add, Sub};

pub trait IndexTree {
    type Item: Copy + Default + Add + Sub;

    fn len(&self) -> usize;

    fn add(&mut self, index: usize, delta: Self::Item);

    fn prefix_query(&self, index: usize) -> Self::Item;

    fn range_query(&self, left_idx: usize, right_idx: usize) -> Self::Item;
}

pub trait BinarySearchTree {
    type Item: Ord;

    fn volume(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.volume() == 0
    }

    fn insert(&mut self, value: Self::Item);

    fn contains(&self, value: &Self::Item) -> bool;

    fn remove(&mut self, value: &Self::Item) -> bool;
}
