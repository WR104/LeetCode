pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] == '0' || visited[i][j] {
            return;
        }

        visited[i][j] = true;

        dfs(grid, visited, i + 1, j);
        dfs(grid, visited, i, j + 1);
        if i > 0 {
            dfs(grid, visited, i - 1, j);
        }
        if j > 0 {
            dfs(grid, visited, i, j - 1);
        }
    }

    if grid.is_empty() {
        return 0;
    }

    let mut count = 0;
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' && !visited[i][j] {
                count += 1;
                dfs(&grid, &mut visited, i, j);
            }
        }
    }

    count
}
