use std::collections::HashMap;

fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    if numerator == 0 {
        return String::from("0");
    }
    let mut res = String::new();
    if (numerator < 0) ^ (denominator < 0) {
        res.push('-');
    }
    let dividend = (numerator as i64).abs();
    let divisor = (denominator as i64).abs();
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    res += &quotient.to_string();
    if remainder == 0 {
        return res;
    }
    res.push('.');
    let mut map = HashMap::new();
    let mut r = remainder;
    while r != 0 {
        if let Some(&i) = map.get(&r) {
            res.insert(i, '(');
            res.push(')');
            break;
        }
        map.insert(r, res.len());
        r *= 10;
        res += &(r / divisor).to_string();
        r %= divisor;
    }
    res
}
