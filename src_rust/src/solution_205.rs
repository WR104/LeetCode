pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut s_map = std::collections::HashMap::new();
    let mut t_map = std::collections::HashMap::new();
    let mut s_chars = s.chars();
    let mut t_chars = t.chars();

    while let (Some(s_c), Some(t_c)) = (s_chars.next(), t_chars.next()) {
        if *s_map.entry(s_c).or_insert(t_c) != t_c || *t_map.entry(t_c).or_insert(s_c) != s_c {
            return false;
        }
    }

    s_chars.next().is_none() && t_chars.next().is_none()
}
