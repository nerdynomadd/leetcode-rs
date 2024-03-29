impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut current_area = 0;

        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            current_area = (right - left) as i32 * std::cmp::min(height[left], height[right]);
            max_area = std::cmp::max(current_area, max_area);

            if(height[left] < height[right]) {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}