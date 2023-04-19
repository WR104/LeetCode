pub fn convert_to_title(column_number: i32) -> String {
    let mut res = String::new();
    let mut n = column_number;
    while n > 0 {
        match (n - 1) % 26 {
            r @ 0..=25 => res.insert(0, (b'A' + r as u8) as char),
            _ => unreachable!(),
        }
        n = (n - 1) / 26;
    }
    res
}
