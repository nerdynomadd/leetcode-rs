impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut x = x;
        if x < 0 {
            return false;
        }
        let mut y = 0;
        let mut z = x;
        while z > 0 {
            y = y * 10 + z % 10;
            z /= 10;
        }
        x == y
    }
}
