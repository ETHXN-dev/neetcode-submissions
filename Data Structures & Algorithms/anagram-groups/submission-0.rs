impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for word in strs {
            let mut l = [0; 26];
            for c in word.chars() {
                l[(c as u8 - b'a') as usize] += 1;
            }

            map.entry(l).or_default().push(word);
        }

        map.into_values().collect()
    }
}
