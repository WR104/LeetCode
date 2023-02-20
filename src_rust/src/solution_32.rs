use std::cmp::max;
use std::collections::VecDeque;

pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut stack = VecDeque::new();
    stack.push_back(-1);
    let mut max_len = 0;

    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            stack.push_back(i as i32);
        } else {
            stack.pop_back();
            if !stack.is_empty() {
                max_len = max(max_len, i as i32 - stack.back().unwrap());
            } else {
                stack.push_back(i as i32);
            }
        }
    }
    max_len
}