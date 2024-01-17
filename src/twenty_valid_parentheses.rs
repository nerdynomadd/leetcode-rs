impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut characters: Vec<char> = vec![];

        for c in s.chars() {
            match c {
                '(' | '[' |'{' => characters.push(c),
                ')' => {
                    if characters.len() == 0 || characters.pop().unwrap() != '(' {
                        return false;
                    }
                },
                ']' => {
                    if characters.len() == 0 || characters.pop().unwrap() != '[' {
                        return false;
                    }
                },
                '}' => {
                    if characters.len() == 0 || characters.pop().unwrap() != '{' {
                        return false;
                    }
                },
                _ => return false,
            }
        }

        characters.len() == 0
    }
}