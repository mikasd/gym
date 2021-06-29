struct Solution;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut max: i32 = 0;

        stack.push(-1);
        s.chars().enumerate().for_each(|(i, c)| {
            if c == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if !stack.is_empty() {
                    let len = i as i32 - stack[stack.len() - 1];
                    max = if max > len { max } else { len };
                } else {
                    stack.push(i as i32);
                }
            }
        });
        max
    }
}