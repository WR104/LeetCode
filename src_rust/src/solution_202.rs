pub fn is_happy(n: i32) -> bool {
    let mut set = std::collections::HashSet::new();
    set.insert(1);

    let mut n = n;
    while !set.contains(&n) {
        set.insert(n);
        n = n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(2))
        .sum::<u32>() as i32;    
    }
    n == 1
}