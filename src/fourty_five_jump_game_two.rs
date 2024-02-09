impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut jumps = 0;
        let mut cur_end = 0;
        let mut cur_farthest = 0;
        for i in 0..n - 1 {
            cur_farthest = cur_farthest.max(i + nums[i as usize] as i32);
            if i == cur_end {
                jumps += 1;
                cur_end = cur_farthest;
            }
        }
        jumps
    }
}