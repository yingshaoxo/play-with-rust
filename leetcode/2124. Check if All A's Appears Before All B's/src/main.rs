pub struct Solution {}

impl Solution {
    pub fn check_string(s: String) -> bool {
        //8:12
        let mut max_a_index = 0;
        let mut min_b_index = s.len();

        if !s.contains("a") && s.contains("b") {
            return true;
        }

        for (i, c) in s.chars().enumerate() {
            if c == 'a' {
                if (i > max_a_index) {
                    max_a_index = i;
                }
            } else if c == 'b' {
                if (i < min_b_index) {
                    min_b_index = i;
                }
            }
        }

        return max_a_index < min_b_index;
        //8:14
        //debug to satisfy edge case until 8:18
    }
}

fn main() {}
