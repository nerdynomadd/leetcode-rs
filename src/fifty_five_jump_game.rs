impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_read = 0;
        for (i, &num) in nums.iter().enumerate() {
            if i > max_read {
                return false;
            }
            max_read = max_read.max((i as usize) + (num as usize));
        }
        true
    }
}