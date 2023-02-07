pub fn is_match(s: String, p: String) -> bool {
	if s.len() == 0 || p.len() == 0 {
		return false
	}
	let m = s.len();
    let n = p.len();
    let mut dp = vec![vec![false; n + 1]; m + 1];
    dp[0][0] = true;

    for j in 1..=n {
        if p.chars().nth(j - 1) == Some('*') {
            dp[0][j] = dp[0][j - 1];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            if p.chars().nth(j - 1) == Some('?') || p.chars().nth(j - 1) == s.chars().nth(i - 1) {
                dp[i][j] = dp[i - 1][j - 1];
            } else if p.chars().nth(j - 1) == Some('*') {
                dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
            }
        }
    }

    dp[m][n]
}