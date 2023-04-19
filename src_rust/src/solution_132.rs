pub fn min_cut(s: String) -> i32 {
    let n = s.len();
    let s = s.into_bytes();
    let mut dp = vec![vec![false; n]; n];
    let mut cuts = vec![0; n];

    for i in (0..n).rev() {
        cuts[i] = n - i - 1;
        for j in i..n {
            if s[i] == s[j] && (j - i < 2 || dp[i+1][j-1]) {
                dp[i][j] = true;
                if j == n - 1 {
                    cuts[i] = 0;
                } else {
                    cuts[i] = cuts[i].min(1 + cuts[j+1]);
                }
            }
        }
    }

    cuts[0] as i32
}