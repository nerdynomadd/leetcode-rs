impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == divisor {
            return 1;
        }

        let is_negative = (dividend < 0) ^ (divisor < 0);
        let (mut dividend, divisor) = (dividend.abs() as u32, divisor.abs() as u32);
        let mut res = 0;

        while dividend >= divisor {
            let mut temp = 0;
            while dividend > (divisor << (temp + 1)) {
                temp += 1;
            }
            res += 1 << temp;
            dividend -= divisor << temp;
        }

        if res == 1 << 31 && !is_negative {
            return i32::MAX;
        }

        match is_negative {
            true => -(res as i32),
            false => res as i32,
        }
    }
}