impl Solution {
    pub fn get_roman_int_representation(str: char) -> i32 {
        match str {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        let chars = s.chars();

        let mut total_number = 0;

        let mut index = 0;

        while index < s.len() {
            let current_roman_representation = Self::get_roman_int_representation(s.chars().nth(index).unwrap());

            if (index + 1  < s.len()) {
                let next_roman_representation = Self::get_roman_int_representation(s.chars().nth(index + 1).unwrap());

                if(current_roman_representation >= next_roman_representation) {
                    total_number += current_roman_representation;
                } else {
                    total_number += next_roman_representation - current_roman_representation;
                    index += 1;
                }
            } else {
                total_number += current_roman_representation;
            }
            index += 1;
        }

        total_number
    }
}