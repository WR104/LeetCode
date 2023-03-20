pub fn restore_ip_addresses(s: String) -> Vec<String> {
    let n = s.len();
    let mut res = Vec::new();
    for i in 1..4 {
        for j in 1..4 {
            for k in 1..4 {
                let l = n - i - j - k;
                if l > 0 && l <= 3 {
                    let a = s[..i].parse::<u8>();
                    let b = s[i..i+j].parse::<u8>();
                    let c = s[i+j..i+j+k].parse::<u8>();
                    let d = s[i+j+k..].parse::<u8>();
                    if a.is_ok() && b.is_ok() && c.is_ok() && d.is_ok() {
                        let ip = format!("{}.{}.{}.{}", a.unwrap(), b.unwrap(), c.unwrap(), d.unwrap());
                        if ip.len() == n + 3 {
                            res.push(ip);
                        }
                    }
                }
            }
        }
    }
    res
}