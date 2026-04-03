pub trait IndexTree {
    type Item: Copy + Default;

    fn len(&self) -> usize;

    fn add(&mut self, index: usize, delta: Self::Item);

    fn prefix_query(&self, index: usize) -> Self::Item;

    fn range_query(&self, left_idx: usize, right_idx: usize) -> Self::Item;
}
