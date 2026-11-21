impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for c in s.chars() {
            if matches!(c, '(' | '{' | '[') {
                stack.push(c);
            } else {
                if (stack.is_empty()) || (c == ')' && stack.pop().unwrap() != '(')
                    || (c == '}' && stack.pop().unwrap() != '{')
                    || (c == ']' && stack.pop().unwrap() != '[')
                {
                    return false;
                }
            }
        }

        true
    }
}
