impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if n == 1 {
            return x;
        }
        if n == -1 {
            return 1.0 / x;
        }
        let half = Self::my_pow(x, n / 2);
        let rest = Self::my_pow(x, n % 2);
        half * half * rest
    }
}