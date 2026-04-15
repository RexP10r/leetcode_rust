use std::cmp::min;

struct Solution;
impl Solution {
    pub fn min_wasted_space(mut packages: Vec<i32>, mut suppliers: Vec<Vec<i32>>) -> i32 {
        packages.sort_unstable();

        let mut res: i64 = i64::MAX;
        let pack_len = packages.len();

        for boxes in suppliers.iter_mut() {
            boxes.sort_unstable();
            if boxes[boxes.len() - 1] < packages[pack_len - 1] {
                continue;
            }
            let mut current_space_used: i64 = 0;
            let mut last_num_covered: usize = 0;
            for &box_size in boxes.iter() {
                let cur_num_covered = packages.partition_point(|&p| p <= box_size);
                current_space_used +=
                    (box_size as i64) * (cur_num_covered - last_num_covered) as i64;
                last_num_covered = cur_num_covered;
            }
            res = min(res, current_space_used);
        }
        match res {
            a if a < i64::MAX => {
                let module: i64 = 1_000_000_007;
                let sum_packages: i64 = packages.iter().map(|&v| v as i64).sum();
                ((a - sum_packages) % module) as i32
            }
            _ => -1,
        }
    }
}
fn main() {
    let _temp = Solution {};
    let temp1 = Solution::min_wasted_space(vec![2, 1, 5, 4], vec![vec![2], vec![10], vec![1, 2]]);
    print!("{}", temp1);
}
