pub struct Solution {}

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        //12:46
        let mut result = String::new();
        let mut one_word = String::new();
        let new_title = title + " ";
        for (_, c) in new_title.chars().enumerate() {
            if c == ' ' {
                if one_word.len() > 2 {
                    let first_char = &one_word.chars().nth(0).unwrap().to_uppercase().to_string();
                    let rest_char = &one_word.chars().skip(1).collect::<String>().to_lowercase();
                    result.push_str(first_char);
                    result.push_str(rest_char);
                } else {
                    result.push_str(&one_word.to_string().to_lowercase());
                }
                one_word.clear();

                if c == ' ' {
                    result.push_str(" ");
                } else {
                    result.push_str(c.to_string().to_lowercase().as_str());
                }
            } else {
                one_word.push(c);
            }
        }
        if result.chars().nth(result.len() - 1).unwrap() == ' ' {
            result.pop();
        }
        return result;
        //12:57
        //debug until 1:14; the github copilot may gave you a wrong answer
    }
}

fn main() {
    let result = Solution::capitalize_title("l CCK n k".to_string());
    println!("{}", result);
}
