impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut s = String::new();
        Self::generate_parenthesis_helper(n, 0, 0, &mut s, &mut result);
        result
    }

    pub fn generate_parenthesis_helper(n: i32, left: i32, right: i32, s: &mut String, result: &mut Vec<String>) {
        if left == n && right == n {
            result.push(s.clone());
            return;
        }

        if left < n {
            s.push('(');
            Self::generate_parenthesis_helper(n, left + 1, right, s, result);
            s.pop();
        }

        if right < left {
            s.push(')');
            Self::generate_parenthesis_helper(n, left, right + 1, s, result);
            s.pop();
        }
    }
}