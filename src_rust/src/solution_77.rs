fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    fn backtrack(n: i32, k: i32, start: i32, combo: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if combo.len() == k as usize {
            res.push(combo.clone());
            return;
        }
        for i in start..=n {
            combo.push(i);
            backtrack(n, k, i+1, combo, res);
            combo.pop();
        }
    }
    let mut res = vec![];
    let mut combo = vec![];
    backtrack(n, k, 1, &mut combo, &mut res);
    res
}