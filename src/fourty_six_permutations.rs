// Solution I - Path construction
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut path = vec![];
        Self::backtrack(&nums, &mut path, &mut res);
        res
    }

    pub fn backtrack(nums: &Vec<i32>, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            res.push(path.clone());
            return;
        }
        for &num in nums {
            if !path.contains(&num) {
                path.push(num);
                Self::backtrack(nums, path, res);
                path.pop();
            }
        }
    }
}

// Solution II - Swapping
impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::backtrack(&mut nums, 0, &mut res);
        res
    }

    pub fn backtrack(nums: &mut [i32], i: usize, res: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            res.push(nums.to_vec());
            return;
        }
        let mut seen = [false; 21];
        for j in i..nums.len() {
            if seen[nums[j] as usize + 10] { continue }
            seen[nums[j] as usize + 10] = true;

            nums.swap(i, j);
            Self::backtrack(nums, i + 1, res);
            nums.swap(i, j);
        }
    }
}