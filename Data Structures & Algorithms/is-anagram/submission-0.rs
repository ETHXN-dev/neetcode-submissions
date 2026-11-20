impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut l1 = [0; 26];
        let mut l2 = [0; 26];

        s.chars().for_each(|x| l1[(x as u8 - b'a') as usize] += 1);
        t.chars().for_each(|x| l2[(x as u8 - b'a') as usize] += 1);

        l1 == l2
    }
}
