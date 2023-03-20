pub fn min_window(s: String, t: String) -> String {
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    let mut count: [i32; 128] = [0; 128];
    for ch in &t {
        count[*ch as usize] += 1;
    }
    println!("{:?}", count);
    let mut curr: [i32; 128] = [0; 128];
    let mut left = 0;
    let mut right = 0;
    let mut found = 0;
    let mut min_len = std::usize::MAX;
    let mut min_start = 0;

    while right < s.len() {
        let ch = s[right];
        curr[ch as usize] += 1;
        if count[ch as usize] > 0 && curr[ch as usize] <= count[ch as usize] {
            found += 1;
        }
        right += 1;

        while found == t.len() {
            if right - left < min_len {
                min_len = right - left;
                min_start = left;
            }
            let ch = s[left];
            curr[ch as usize] -= 1;
            if count[ch as usize] > 0 && curr[ch as usize] < count[ch as usize] {
                found -= 1;
            }
            left += 1;
        }
    }

    if min_len == std::usize::MAX {
        return "".to_string();
    } else {
        return s[min_start..min_start+min_len].iter().collect();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_76() {
        assert_eq!(min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC".to_string());
        assert_eq!(min_window("a".to_string(), "a".to_string()), "a");
        assert_eq!(min_window("a".to_string(), "aa".to_string()), "");
        assert_eq!(min_window("a".to_string(), "b".to_string()), "");
        assert_eq!(min_window("ab".to_string(), "a".to_string()), "a");
        assert_eq!(min_window("abc".to_string(), "bc".to_string()), "bc");
        assert_eq!(min_window("aabaabaaab".to_string(), "bb".to_string()), "baab");
        assert_eq!(min_window("bba".to_string(), "ab".to_string()), "ba");
    }
}