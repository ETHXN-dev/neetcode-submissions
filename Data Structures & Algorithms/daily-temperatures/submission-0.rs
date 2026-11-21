impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(&i32, usize)> = vec![];
        let mut res = vec![0; temperatures.len()];

        for (i, temp) in temperatures.iter().enumerate() {
            while !stack.is_empty() {
                if temp > stack.last().unwrap().0 {
                    let j = stack.pop().unwrap().1;
                    res[j] = (i - j) as i32;
                } else {
                    break;
                }
            }
            stack.push((temp, i));
        }

        res
    }
}
