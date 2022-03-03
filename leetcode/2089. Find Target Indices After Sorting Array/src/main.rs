pub struct Solution {}

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        //10:30
        let mut new_nums = nums.clone();
        new_nums.sort_by(|a, b| a.cmp(b));

        let mut index_list: Vec<i32> = Vec::new();
        for (i, &num) in new_nums.iter().enumerate() {
            if num == target {
                index_list.push(i as i32);
            }
        }

        index_list.sort_by(|a, b| a.cmp(b));
        return index_list;
        //10:33
        //debug until 10:35
    }
}

fn main() {}
