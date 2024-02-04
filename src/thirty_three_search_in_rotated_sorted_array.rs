// Simple binary search
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // As we are sure to get only distinct values, we can use a simple binary search
        let mut size = nums.len();
        for i in 0..size {
            if nums[i] == target {
                return i as i32;
            }
        }
        -1
    }
}

// Split binary search
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // Find the pivot point and make a split comparaison
        // To find the pivot point, and following the properties of the array passed as an input, we are sure that the pivot comes were the
        // Value at index i becomes less than the following index
        let mut pivot = 0;
        let mut size = nums.len();

        if size == 1 {
            return if nums[0] == target { 0 } else { -1 };
        }

        for i in 0..size {
            if nums[i] > nums[i + 1] {
                pivot = i + 1;
                break;
            }
        }

        // Now we can make a split comparison
        let mut left = 0;
        let mut right = size - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            let real_mid = (mid + pivot) % size;
            if nums[real_mid] == target {
                return real_mid as i32;
            } else if nums[real_mid] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        -1
    }
}