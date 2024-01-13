impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::new();
        } else if strs.len() == 1 {
            return strs[0].clone();
        }

        // Select an arbritary word as the searched substring should be in any word of the Vector
        let base_word = &strs[0];

        let mut search_substring = String::new();
        let mut r#match = String::new();

        for (i, char) in base_word.chars().enumerate() {
            if search_substring.is_empty() && i != 0 {
                break;
            }

            // Grow the search_substring as we go through the iterations
            search_substring.push(char);

            // Iterate through all the other strings to check if our searching substring matches any start characters
            for string in &strs[1..] {
                if string.starts_with(&search_substring) {
                    if search_substring.len() > r#match.len() {
                        r#match = String::from(&search_substring);
                    }
                } else {
                    if search_substring.len() == r#match.len() && search_substring.contains(&r#match) {
                        r#match.pop();
                    }

                    search_substring.clear();

                    break;
                }
            }
        }

        r#match
    }
}