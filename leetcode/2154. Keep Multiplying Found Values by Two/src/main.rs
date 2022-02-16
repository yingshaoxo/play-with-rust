pub struct Solution {}

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        //8:12
        let mut my_original = original.clone();
        let mut keep_loop = true;
        while keep_loop {
            let mut found = false;
            for i in 0..nums.len() {
                if nums[i] == my_original {
                    found = true;
                    my_original = nums[i] * 2;
                }
            }
            if found == false {
                keep_loop = false;
            } else {
                keep_loop = true;
            }
        }
        return my_original;
        //8:15
    }
}

fn main() {}
