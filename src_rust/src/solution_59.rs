pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; n as usize]; n as usize];
    let mut value = 1;
    let (mut top, mut bottom, mut left, mut right) = (0, n - 1, 0, n - 1);
    while top <= bottom && left <= right {
        for j in left..=right {
            matrix[top as usize][j as usize] = value;
            value += 1;
        }
        top += 1;

        for i in top..=bottom {
            matrix[i as usize][right as usize] = value;
            value += 1;
        }
        right -= 1;

        if top <= bottom {
            for j in (left..=right).rev() {
                matrix[bottom as usize][j as usize] = value;
                value += 1;
            }
            bottom -= 1;
        }

        if left <= right {
            for i in (top..=bottom).rev() {
                matrix[i as usize][left as usize] = value;
                value += 1;
            }
            left += 1;
        }
    }

    matrix
}