use std::collections::BinaryHeap;

// BinaryHep solution
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = BinaryHeap::new();
        let mut max = 0;
        let mut last = -1;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(i as i32);
            } else {
                if stack.is_empty() {
                    last = i as i32;
                } else {
                    stack.pop();
                    if stack.is_empty() {
                        max = max.max(i as i32 - last);
                    } else {
                        max = max.max(i as i32 - stack.peek().unwrap());
                    }
                }
            }
        }
        max
    }
}

// Vec solution
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![-1];
        let mut max = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    max = max.max(i as i32 - stack.last().unwrap());
                }
            }
        }
        max
    }
}

// Sized array solution
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack: [i32; 32] = [-1];
        let mut max = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    max = max.max(i as i32 - stack.last().unwrap());
                }
            }
        }
        max
    }
}