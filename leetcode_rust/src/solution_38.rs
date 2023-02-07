pub fn count_and_say(n: i32) -> String {
    let mut res = String::from("1");
    for _ in 1..n {
        res = helper(res);
    }
    res
}

pub fn helper(num: String) -> String {
    let mut result = String::new();
    let mut last_char = num.chars().nth(0).unwrap();
    let mut current_group = 1;
    for c in num.chars().skip(1) {
        if c == last_char {
            current_group += 1;
        } else {
            result.push_str(&current_group.to_string());
            result.push(last_char);
            current_group = 1;
            last_char = c;
        }
    }
    result.push_str(&current_group.to_string());
    result.push(last_char);
    result
}