pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace().last().map(|s| s.len()).unwrap() as i32
}