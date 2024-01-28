impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut starting_indices = Vec::new();
        let word_length = words[0].len();
        let mut word_count = words.len();
        let mut word_map = std::collections::HashMap::new();
        for word in words {
            *word_map.entry(word).or_insert(0) += 1;
        }
        for i in 0..s.len() - word_length * word_count + 1 {
            let mut seen = std::collections::HashMap::new();
            let mut j = 0;
            while j < word_count {
                let word = &s[i + j * word_length..i + (j + 1) * word_length];
                if word_map.contains_key(word) {
                    *seen.entry(word).or_insert(0) += 1;
                    if seen[word] > word_map[word] {
                        break;
                    }
                } else {
                    break;
                }
                j += 1;
            }
            if j == word_count {
                starting_indices.push(i as i32);
            }
        }
        starting_indices
    }
}