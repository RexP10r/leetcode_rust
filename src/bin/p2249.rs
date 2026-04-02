use std::collections::HashSet;

struct Solution;

impl Solution {
    fn is_point_lattice((p_x, p_y): &(i32, i32), (x, y, rad): &(i32, i32, i32)) -> bool {
        let dx = p_x - x;
        let dy = p_y - y;
        dx * dx + dy * dy <= rad * rad
    }
    fn insert_circle(points: &mut HashSet<(i32, i32)>, circle: &[i32]) {
        let [x, y, rad] = circle.try_into().unwrap();

        for i in -rad..=rad {
            for j in -rad..=rad {
                let p_x: i32 = x + i;
                let p_y: i32 = y - j;
                if points.contains(&(p_x, p_y)) {
                    continue;
                }
                if Self::is_point_lattice(&(p_x, p_y), &(x, y, rad)) {
                    points.insert((p_x, p_y));
                }
            }
        }
    }
    pub fn count(circles: Vec<Vec<i32>>) -> i32 {
        let mut points: HashSet<(i32, i32)> = HashSet::new();
        for circle in circles {
            Self::insert_circle(&mut points, &circle);
        }
        points.len() as i32
    }
}

fn main() {
    let circle = vec![vec![1,1,1]];
    let result = Solution::count(circle);
    println!("Circle [1,1,1] has {} lattice points", result);
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn one_cirle() {
        let circle: Vec<Vec<i32>> = vec![vec![2, 2, 1]];
        let result = Solution::count(circle);
        assert_eq!(result, 5);
    }
}
