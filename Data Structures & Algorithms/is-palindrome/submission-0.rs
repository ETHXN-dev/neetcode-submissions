impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut letters = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase());

        while let (Some(front), Some(back)) = (letters.next(), letters.next_back()) {
            if front != back {
                return false;
            }
        }

        true
    }
}
