impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        // For each string, sort the characters and use the sorted string as a key into the map
        for s in strs {
            let mut key = s.chars().collect::<Vec<char>>();
            key.sort();
            let key = key.into_iter().collect::<String>();
            map.entry(key).or_insert(Vec::new()).push(s);
        }
        map.into_iter().map(|(_, v)| v).collect()
    }
}