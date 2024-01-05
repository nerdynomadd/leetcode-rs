impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = if x.is_negative() { -1 } else { 1 };
        let abs_x = x.abs().to_string().chars().rev().collect::<String>();
        abs_x.parse::<i32>().unwrap_or(0) * sign
    }
}