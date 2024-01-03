impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&num) {
                return vec![j as i32, i as i32];
            }
            map.insert(target - num, i);
        }
        unreachable!()
    }
}
