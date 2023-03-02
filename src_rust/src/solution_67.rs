pub fn add_binary(a: String, b: String) -> String {
    let mut carry = 0;
    let mut sum = String::new();
    let (mut i, mut j) = (a.len(), b.len());
    while i.checked_sub(1).is_some() || j.checked_sub(1).is_some() || carry == 1 {
        let ai = if i > 0 { a.as_bytes()[i - 1] - b'0' } else { 0 };
        let bj = if j > 0 { b.as_bytes()[j - 1] - b'0' } else { 0 };
        let mut s = ai + bj + carry;
        carry = s / 2;
        s %= 2;
        sum.push(char::from_digit(s as u32, 10).unwrap());
        i = i.checked_sub(1).unwrap_or(0);
        j = j.checked_sub(1).unwrap_or(0);
    }
    let sum = sum.chars().rev().collect::<String>();
    sum
}