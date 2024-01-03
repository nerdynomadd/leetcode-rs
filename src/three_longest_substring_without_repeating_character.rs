impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;

        let mut pos: [usize;128] = [0;128];

        let mut start: usize = 0;

        for (end, ch) in s.chars().enumerate() {
            start = start.max(pos[ch as usize]);
            max = max.max(end - start + 1);
            pos[ch as usize] = end + 1;
        }

        max as i32
    }
}
