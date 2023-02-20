pub fn unique_paths_with_obstacles(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut table = vec![vec![0; n]; m];
    table[0][0] = 1 - grid[0][0];
    for i in 1..m {
        table[i][0] = table[i-1][0] * (1 - grid[i][0]);
    }
    for j in 1..n {
        table[0][j] = table[0][j-1] * (1 - grid[0][j]);
    }
    for i in 1..m {
        for j in 1..n {
            table[i][j] = (table[i-1][j] + table[i][j-1]) * (1 - grid[i][j]);
        }
    }
    table[m-1][n-1]
}