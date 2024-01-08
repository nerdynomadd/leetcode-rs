
use std::fmt::{Debug, Display};

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // Create a new matrix of size s.len() + 1 and p.len() + 1 to store the association between s and p with an initial line and column representing an empty string.
        let mut association_matrix = Matrix::new(false, s.len() + 1, p.len() + 1);

        // In the first element at index (0, 0) put the value True because it's "0" and "0". So both strings are empty ones, thus they match.
        association_matrix.get_mut(0, 0).map(|v| *v = true);

        let s = s.as_bytes();
        let p = p.as_bytes();

        // Fill the first line of the matrix with the value of the previous column if the current pattern's character is a star.
        for i in 2..=p.len() {
            if(p[i - 1] == b'*') {
                let value = association_matrix.get(0, i - 2).or(Some(&false)).unwrap().clone();
                association_matrix.get_mut(0, i).map(|v| *v = value);
            }
        }

        // Fill the matrix with the association between s and p.
        for i in 1..=s.len() {
            for j in 1..=p.len() {
                // If the current pattern's character matches the current one of the string, or if the current pattern's character is a global match
                if s[i - 1] == p[j - 1] || p[j - 1] == b'.' {
                    let value = association_matrix.get(i - 1, j - 1).or(Some(&false)).unwrap().clone();
                    association_matrix.get_mut(i, j).map(|v| *v = value);
                // If the current pattern's character is a star
                } else if p[j - 1] == b'*' {
                    let value = association_matrix.get(i, j - 2).or(Some(&false)).unwrap().clone();
                    association_matrix.get_mut(i, j).map(|v| *v = value);
                    // If the previous pattern's character matches the current one of the string, or if the previous pattern's character is a global match
                    if p[j - 2] == s[i - 1] || p[j - 2] == b'.' {
                        let value = association_matrix.get(i - 1, j).or(Some(&false)).unwrap().clone() || association_matrix.get(i, j - 2).or(Some(&false)).unwrap().clone();
                        association_matrix.get_mut(i, j).map(|v| *v = value);
                    }
                }
            }
        }

        association_matrix.get(s.len(), p.len()).unwrap_or(&false).clone()
    }
}

#[derive(Debug)]
pub struct Matrix<T> where T: Display + Clone {
    matrix: Vec<Vec<T>>,
}

impl<T: Display + Clone> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for row in &self.matrix {
            for col in row {
                result.push_str(&format!("{}, ", col));
            }
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}

impl<T: Display + Clone> Matrix<T> {
    pub fn new(default: T, row: usize, col: usize) -> Self where T: Clone {
        let mut matrix = Vec::with_capacity(row);
        for _ in 0..row {
            let mut row = Vec::with_capacity(col);
            for _ in 0..col {
                row.push(default.clone());
            }
            matrix.push(row);
        }
        Self {
            matrix
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.matrix.get(row).and_then(|row| row.get(col))
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.matrix.get_mut(row).and_then(|row| row.get_mut(col))
    }
}