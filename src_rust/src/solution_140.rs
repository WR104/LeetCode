pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    fn build_sentences(
        i: usize,
        prev: &[Vec<usize>],
        s: &str,
        sentence: &mut Vec<String>,
        res: &mut Vec<String>,
    ) {
        if i == 0 {
            res.push(sentence.iter().rev().map(|w| w.to_owned()).collect::<Vec<_>>().join(" "));
            return;
        }

        for j in &prev[i] {
            let word = &s[*j..i];
            sentence.push(word.to_owned());
            build_sentences(*j, prev, s, sentence, res);
            sentence.pop();
        }
    }
    let n = s.len();
    let mut dp = vec![false; n + 1];
    let mut prev = vec![Vec::new(); n + 1];
    dp[0] = true;

    for i in 1..=n {
        for j in (0..i).rev() {
            if dp[j] && word_dict.contains(&s[j..i].to_string()) {
                dp[i] = true;
                prev[i].push(j);
            }
        }
    }

    let mut res = Vec::new();
    let mut sentence = Vec::new();
    build_sentences(n, &prev, &s, &mut sentence, &mut res);
    res
}