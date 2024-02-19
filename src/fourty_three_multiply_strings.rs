impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        let mut res = vec![0; num1.len() + num2.len()];
        // Collect numbers as vector of digits
        let num1: Vec<u8> = num1.bytes().map(|b| b - b'0').collect();
        let num2: Vec<u8> = num2.bytes().map(|b| b - b'0').collect();
        // Multiply each digit and add to the result
        for (i, &n1) in num1.iter().rev().enumerate() {
            for (j, &n2) in num2.iter().rev().enumerate() {
                let sum = res[i + j] + n1 * n2;
                res[i + j] = sum % 10;
                res[i + j + 1] += sum / 10;
            }
        }
        // Delete leading zeros
        while let Some(&0) = res.last() {
            res.pop();
        }
        // Convert to string
        res.into_iter().rev().map(|n| (n + b'0') as char).collect()
    }
}