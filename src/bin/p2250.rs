use leetcode_rust::ds::{fenwick_tree::FenwickSumTree, traits::IndexTree};

pub struct Solution;

impl Solution {
    pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
        let mut rects: Vec<(i32, i32)> = rectangles.into_iter().map(|r| (r[0], r[1])).collect();
        rects.sort_unstable_by_key(|r| std::cmp::Reverse(r.0));

        let mut pts: Vec<(i32, i32, usize)> = points
            .into_iter()
            .enumerate()
            .map(|(i, p)| (p[0], p[1], i))
            .collect();
        pts.sort_unstable_by_key(|p| std::cmp::Reverse(p.0));

        let mut bit = FenwickSumTree::new(100);
        let mut ans = vec![0; pts.len()];
        let mut r_idx = 0;

        for &(x, y, orig_idx) in &pts {
            while r_idx < rects.len() && rects[r_idx].0 >= x {
                bit.add(rects[r_idx].1 as usize, 1);
                r_idx += 1;
            }
            ans[orig_idx] = bit.range_query(y as usize, 100);
        }
        ans
    }
}
fn main() {
    let output =
        Solution::count_rectangles(vec![vec![33, 3], vec![33, 5]], vec![vec![4, 5], vec![4, 1]]);
    println!("Just: {:#?}", output);
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn base() {
        let rectangles = vec![vec![1, 2], vec![2, 3], vec![2, 5]];
        let points = vec![vec![2, 1], vec![1, 4]];
        let output = Solution::count_rectangles(rectangles, points);

        assert_eq!(output, vec![2, 1]);
    }
}
