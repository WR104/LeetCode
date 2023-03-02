impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut col = vec![];
        let mut row = vec![];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    col.push(i);
                    row.push(j);
                }
            }
        }
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if col.contains(&i) || row.contains(&j) {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

