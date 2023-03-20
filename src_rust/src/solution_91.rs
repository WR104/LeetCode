pub fn num_decodings(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    let mut dp = vec![0; s.len()+1];
    dp[0] = 1;
    dp[1] = if s[0] == '0' { 0 } else { 1 };
    for i in 2..=s.len() {
        if s[i-1] != '0' {
            dp[i] += dp[i-1];
        }
        let num = (s[i-2] as u8 - b'0') * 10 + (s[i-1] as u8 - b'0');
        if num >= 10 && num <= 26 {
            dp[i] += dp[i-2];
        }
    }
    dp[s.len()]
}