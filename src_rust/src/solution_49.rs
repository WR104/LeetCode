use std::collections::HashMap;
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for s in strs {
        let sorted = sort_string(&s);
        map.entry(sorted).or_insert(Vec::new()).push(s);
    }
    
    map.into_values().collect::<Vec<_>>()
}

fn sort_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.into_iter().collect()
}