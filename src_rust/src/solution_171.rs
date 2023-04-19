pub fn title_to_number(column_title: String) -> i32 {
    column_title
        .bytes()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, b)| acc + ((b - b'A' + 1) as i32 * 26_i32.pow(i as u32)))
}