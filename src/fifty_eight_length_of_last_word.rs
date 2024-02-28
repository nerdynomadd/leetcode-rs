impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        // Split the string by space and collect the last word
        let last_word = s.split_whitespace().last().unwrap();

        // Return the length of the last word
        last_word.len() as i32
    }
}