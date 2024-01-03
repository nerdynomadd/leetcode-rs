impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // Implement of the manacher's algorithm
        let mut bytes = Vec::new();
        for b in s.bytes() {
            bytes.push(b'|');
            bytes.push(b)
        }
        bytes.push(b'|');

        let len = bytes.len();
        let mut radii = vec![0;len];
        let mut center = 0;
        let mut radius = 0;

        while center < len {
            while center >= radius + 1 && center + (radius + 1) < len && bytes[center - (radius + 1)] == bytes[center + (radius + 1)] {
                radius += 1;
            }

            radii[center] = radius;

            let old_center = center;
            let old_radius = radius;
            center += 1;
            radius = 0;

            while center <= old_center + old_radius {
                let mirror_center = old_center - (center - old_center);
                let max_mirror_radius = old_center + old_radius - center;

                if radii[mirror_center] < max_mirror_radius {
                    radii[center] = radii[mirror_center];
                    center += 1
                }
                else if radii[mirror_center] > max_mirror_radius {
                    radii[center] = max_mirror_radius;
                    center += 1
                }
                else {
                    radius = max_mirror_radius;
                    break;
                }
            }
        }

        // Search for the max_radius and max_center into the raadi vector
        let (max_radius, max_center) = Self::find_longest_palindrome(radii);

        let mut answer = String::new();
        for i in (max_center - max_radius + 1..max_center + max_radius).step_by(2) {
            answer.push(bytes[i] as char);
        }

        answer
    }

    pub fn find_longest_palindrome(radii: Vec<usize>) -> (usize, usize) {
        let mut max_radius = 0;
        let mut max_center = 0;
        for (i, r) in radii.iter().enumerate() {
            if *r > max_radius {
                max_radius = *r;
                max_center = i;
            }
        }
        (max_radius, max_center)
    }
}