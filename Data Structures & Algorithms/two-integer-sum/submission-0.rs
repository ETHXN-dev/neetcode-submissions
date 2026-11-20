impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            let complement = target - n;

            if seen.contains_key(&complement) {
                return vec![*seen.get(&complement).unwrap(), i as i32];
            }
            seen.insert(n, i as i32);
        }
        unreachable!()
    }
}
