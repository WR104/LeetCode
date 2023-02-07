pub fn multiply(num1: String, num2: String) -> String {
    let n1 = num1.chars().rev().collect::<Vec<_>>();
    let n2 = num2.chars().rev().collect::<Vec<_>>();
    let mut result = vec![0; n1.len() + n2.len()];

    for i in 0..n1.len() {
        for j in 0..n2.len() {
            result[i + j] += (n1[i] as i32 - 48) * (n2[j] as i32 - 48);
            result[i + j + 1] += result[i + j] / 10;
            result[i + j] %= 10;
        }
    }

    while result.len() > 1 && result.last().unwrap() == &0 {
        result.pop();
    }

    result.iter().rev().map(|d| (d + 48) as u8 as char).collect()
}