// 6ms solution
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = "1".to_string();
        for _ in 1..n {
            let mut temp = String::new();
            let mut i = 0;
            while i < result.len() {
                let mut count = 1;
                while i + 1 < result.len() && result.chars().nth(i).unwrap() == result.chars().nth(i + 1).unwrap() {
                    i += 1;
                    count += 1;
                }
                temp.push_str(&count.to_string());
                temp.push(result.chars().nth(i).unwrap());
                i += 1;
            }
            result = temp;
        }
        result
    }
}