pub struct Solution {}

impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        //8:50
        let num_string = num.to_string();
        let mut number_vector: Vec<i32> = Vec::new();
        for (_, value) in num_string.chars().enumerate() {
            let single_num = value.to_string().parse::<i32>().unwrap();
            number_vector.push(single_num);
        }
        number_vector.sort();

        let number_a = (number_vector[0].to_string() + &number_vector[2].to_string())
            .parse::<i32>()
            .unwrap();
        let number_b = (number_vector[1].to_string() + &number_vector[3].to_string())
            .parse::<i32>()
            .unwrap();

        return number_a + number_b;
        // 9:02
        // feel so bad since I can't use third party package in leetcode, for example, no way to use combinations
    }
}

fn main() {
    let result = Solution::minimum_sum(1234);
    println!("{}", result);
}
