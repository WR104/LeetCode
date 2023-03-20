pub fn gray_code(n: i32) -> Vec<i32> {
    let mut result = vec![0];

    for i in 0..n {
        let mask = 1 << i;
        let len = result.len();

        for j in (0..len).rev() {
            result.push(result[j] | mask);
        }
    }

    result
}
