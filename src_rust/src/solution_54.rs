pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
	let (m_len, n_len) = (matrix.len(), matrix[0].len());
    let mut matrix = matrix;
    let mut res = vec![];
    let direction: Vec<Vec<i32>> = vec![vec![0,1],vec![1,0],vec![0,-1],vec![-1,0]];
    let mut index = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;

    for _ in 0..m_len * n_len {
        res.push(matrix[m as usize][n as usize]);
        matrix[m as usize][n as usize] = -101;
        let next_m = m + direction[index][0];
        let next_n = n + direction[index][1];
        if next_m < 0 || next_m >= m_len as i32 || next_n < 0 || next_n >= n_len as i32 || matrix[next_m as usize][next_n as usize] == -101 {
            index = (index + 1) % 4;
        }
        m += direction[index][0];
        n += direction[index][1];
    }

    res
}