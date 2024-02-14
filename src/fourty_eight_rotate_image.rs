impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n /2 {
            for j in i..n - i - 1 {
                // Store current cell in a temporary value, as it'll be overwritten
                let temp = matrix[i][j];
                // Move values from right to top
                matrix[i][j] = matrix[n - j - 1][i];
                // Move values from bottom to right
                matrix[n - j - 1][i] = matrix[n - i - 1][n - j - 1];
                // Move values from left to bottom
                matrix[n - i - 1][n - j - 1] = matrix[j][n - i - 1];
                // Assign temp to left
                matrix[j][n - i - 1] = temp;
            }
        }
    }
}