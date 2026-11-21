impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        for token in tokens {
            let token = token.as_str();

            if matches!(token, "+" | "-" | "/" | "*") {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();

                match token {
                    "+" => stack.push(b + a),
                    "-" => stack.push(b - a),
                    "*" => stack.push(b * a),
                    "/" => stack.push(b / a),
                    _ => (),
                }
            } else {
                stack.push(token.parse().unwrap());
            }
        }

        stack[0]
    }
}
