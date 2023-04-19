pub fn partition(s: String) -> Vec<Vec<String>> {
    fn dfs(
        s: &String,
        path: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
        dp: &Vec<Vec<bool>>,
        start: usize,
    ) {
        if start == s.len() {
            result.push(path.clone());
            return;
        }

        for end in start..s.len() {
            if dp[start][end] {
                let substr = s[start..=end].to_owned();
                path.push(substr);
                dfs(s, path, result, dp, end + 1);
                path.pop();
            }
        }
    }
    let mut result = Vec::new();
    let n = s.len();
    let mut dp: Vec<Vec<bool>> = vec![vec![false; n]; n];

    for j in 0..n {
        for i in 0..=j {
            if s.chars().nth(i) == s.chars().nth(j) && (j - i <= 2 || dp[i + 1][j - 1]) {
                dp[i][j] = true;
            }
        }
    }

    dfs(&s, &mut Vec::new(), &mut result, &dp, 0);
    result
}