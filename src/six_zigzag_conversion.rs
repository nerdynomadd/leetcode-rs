impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut resulting_string = String::new();
        let mut rows: Vec<String> = vec![String::new(); num_rows as usize];

        // The zigzag pattern goes as follows:
        // |    /|
        // |  /  |
        // |/    |
        // So we need to take the modulo of the index with the number of rows * 2 - 2 (the number of characters in a zigzag pattern)
        for (i, c) in s.chars().enumerate() {
            if num_rows == 1 {
                return s;
            }
            let row = i % ((2 * num_rows - 2) as usize);
            if row < num_rows as usize {
                rows[row].push(c);
            } else {
                rows[2 * num_rows as usize - row - 2].push(c);
            }
        }

        // Reconstruct the string
        rows.join("")
    }
}
