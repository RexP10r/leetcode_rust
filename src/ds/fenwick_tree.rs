pub struct FenwickSumTree {
    tree: Vec<i32>,
}

impl FenwickSumTree {
    pub fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    pub fn add(&mut self, mut i: usize, delta: i32) {
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    pub fn sum(&self, mut i: usize) -> i32 {
        let mut result = 0;
        while i > 0 {
            result += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        result
    }

    pub fn range_sum(&self, left: usize, right: usize) -> i32 {
        if left == 0 {
            return self.sum(right);
        }
        self.sum(right) - self.sum(left-1)
    }
}

