impl Solution {
    const ROMAN_1: [&'static str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    const ROMAN_10: [&'static str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    const ROMAN_100: [&'static str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    const ROMAN_1000: [&'static str; 4] = ["", "M", "MM", "MMM"];

    pub fn int_to_roman(num: i32) -> String {
        format!("{}{}{}{}",
                Self::ROMAN_1000[(num / 1000 % 10) as usize],
                Self::ROMAN_100[(num / 100 % 10) as usize],
                Self::ROMAN_10[(num / 10 % 10) as usize],
                Self::ROMAN_1[(num % 10) as usize])
    }
}
