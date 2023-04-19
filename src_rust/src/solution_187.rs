use std::collections::HashMap;

pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    if s.len() < 10 {
        return Vec::new();
    }
    let mut map = HashMap::new();
    let mut res = Vec::new();
    for i in 0..s.len() - 9 {
        let seq = &s[i..i + 10];
        *map.entry(seq).or_insert(0) += 1;
        if map[seq] == 2 {
            res.push(seq.to_string());
        }
    }
    res
}
