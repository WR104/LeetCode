fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let max_width = max_width as usize;
    let mut lines = Vec::new();
    let mut i = 0;
    while i < words.len() {
        let mut j = i + 1;
        let mut width = words[i].len();
        while j < words.len() && width + words[j].len() + (j - i - 1) < max_width {
            width += words[j].len();
            j += 1;
        }
        let mut line = words[i].to_string();
        let mut spaces = j - i - 1;
        if j == words.len() || spaces == 0 {
            for k in i + 1..j {
                line.push(' ');
                line += &words[k];
            }
            for _ in 0..max_width - line.len() {
                line.push(' ');
            }
        } else {
            let spaces_per_word = (max_width - width) / spaces;
            let extra_spaces = (max_width - width) % spaces;
            for k in i + 1..j {
                for _ in 0..spaces_per_word {
                    line.push(' ');
                }
                if extra_spaces > 0 {
                    line.push(' ');
                    extra_spaces -= 1;
                }
                line += &words[k];
            }
        }
        lines.push(line);
        i = j;
    }
    lines
}