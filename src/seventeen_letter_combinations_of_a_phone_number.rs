impl Solution {
    pub const MAPPING: [[Option<&'static str>; 4]; 8] = [
        [Some("a"), Some("b"), Some("c"), None],
        [Some("d"), Some("e"), Some("f"), None],
        [Some("g"), Some("h"), Some("i"), None],
        [Some("j"), Some("k"), Some("l"), None],
        [Some("m"), Some("n"), Some("o"), None],
        [Some("p"), Some("q"), Some("r"), Some("s")],
        [Some("t"), Some("u"), Some("v"), None],
        [Some("w"), Some("x"), Some("y"), Some("z")],
    ];

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let numbers: Vec<i32> = digits.chars().map(|c| c.to_digit(10).unwrap() as i32 - 2).collect();

        let mut result: Vec<String> = vec![];

        if numbers.len() == 0 {
            return result;
        }

        // Fill in the result vec with a combinaison of all the letters associated with each digit, for 23 we get ad ae af bd be bf cd ce cf
        for i in 0..numbers.len() {
            let mut temp = vec![];

            for j in 0..Solution::MAPPING[numbers[i] as usize].len() {
                if let Some(letter) = Solution::MAPPING[numbers[i] as usize][j] {
                    if result.len() == 0 {
                        temp.push(letter.to_string());
                    } else {
                        for k in 0..result.len() {
                            temp.push(format!("{}{}", result[k], letter));
                        }
                    }
                }
            }

            result = temp;
        }

        result.sort();

        result
    }
}