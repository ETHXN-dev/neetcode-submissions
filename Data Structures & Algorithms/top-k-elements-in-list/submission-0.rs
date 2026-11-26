impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut frequency_count: HashMap<i32, usize> = HashMap::new();

        for num in &nums {
            *frequency_count.entry(*num).or_default() += 1;
        }

        let mut buckets: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];

        for (num, count) in frequency_count {
            buckets[count].push(num);
        }

        buckets
            .iter()
            .rev()
            .flatten()
            .copied()
            .take(k as usize)
            .collect()
    }
}
