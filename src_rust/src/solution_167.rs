pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    for (i, x) in numbers.iter().enumerate() {
        if let Some(j) = map.get(&(target - x)) {
            return vec![*j as i32 + 1, i as i32 + 1];
        }
        map.insert(x, i);
    }
    vec![]
}
