impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        // Test a combination of 3 numbers which their sum is closest to target
        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum == target {
                    return target;
                } else if sum < target {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
    }
}