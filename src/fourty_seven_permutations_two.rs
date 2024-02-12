// Solution I - Swapping
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
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

// Solution II - Path construction
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut path = vec![];
        let mut nums = nums;
        nums.sort();
        Self::backtrack(&nums, &mut path, &mut res, 0);
        res
    }

    pub fn backtrack(nums: &Vec<i32>, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, i: usize) {
        if path.len() == nums.len() {
            res.push(path.clone());
            res
        }


    }
}