pub fn total_n_queens(n: i32) -> i32 {
    let mut res = 0;
    let m = vec![vec![0; n as usize]; n as usize];
    helper1(m, &mut res, 0);
    res  
}

fn helper1(m: Vec<Vec<usize>>, res: &mut i32, level: usize) {
    let n = m.len();
    for i in 0..n {
        if m[level][i] == 0 {
            let mut new_m = m.clone();
            new_m[level][i] = 1;
            for s in 1..n - level {
                new_m[level + s][i] += 2;
                if i + s < n {
                    new_m[level + s][i + s] += 2
                };
                if i >= s {
                    new_m[level + s][i - s] += 2
                };
            }
            if level == n - 1 {
                *res += 1;
            } else {
                helper1(new_m, res, level + 1);
            }
        }
    }
}