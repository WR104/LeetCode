pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut res = vec![vec![0;n];m];      
    res[0][0] = grid[0][0];
    for i in 1..n {
        res[0][i] = grid[0][i] + res[0][i-1];
    }
    for i in 1..m {
        res[i][0] = grid[i][0] + res[i-1][0];
    }

    for mm in 1..m {
        for nn in 1..n {
            res[mm][nn] = res[mm-1][nn].min(res[mm][nn-1]) + grid[mm][nn];
        }
    }
    res[m-1][n-1]
}