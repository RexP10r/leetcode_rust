struct Solution;
impl Solution {
    fn find_max_variance_for_symbols(s: &String, (a, b): &(u8, u8)) -> i16 {
        let mut global_variance: i16 = 0;
        let mut current_variance: i16 = 0;
        let mut reset = false;
        let mut has_b = false;
        for c in s.as_bytes() {
            if *c == *a {
                current_variance += 1;
            } else if *c == *b {
                has_b = true;
                if current_variance >= 0 && reset {
                    reset = false;
                } else if current_variance <= 0 {
                    reset = true;
                    current_variance = -1;
                } else {
                    current_variance -= 1;
                }
            }

            global_variance = global_variance.max(if has_b { current_variance } else { 0 });
        }
        global_variance
    }
    pub fn largest_variance(s: String) -> i32 {
        let mut present = [false; 26];
        for &b in s.as_bytes() {
            present[(b - b'a') as usize] = true;
        }

        let mut unique_chars = Vec::with_capacity(26);
        for i in 0..26 {
            if present[i] {
                unique_chars.push(i as u8 + b'a');
            }
        }
        print!("{}\n", s);
        print!("{:?}\n", unique_chars);
        let mut res: i16 = 0;
        for &a in unique_chars.iter() {
            for &b in unique_chars.iter() {
                if a != b {
                    res = res.max(Self::find_max_variance_for_symbols(&s, &(a, b)));
                }
            }
        }
        res as i32
    }
}
fn main() {
    let _ = Solution;
    let temp = Solution::largest_variance(String::from("aababbb"));
    print!("\n{}", temp);
}
