pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut res = vec![];
    let m = vec![vec![0; n as usize]; n as usize];
    helper1(m, &mut res, 0);
    res
}

fn helper1(m: Vec<Vec<usize>>, res: &mut Vec<Vec<String>>, level: usize) {
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
                res.push(convert(new_m));
            } else {
                helper1(new_m, res, level + 1);
            }
        }
    }
}

fn convert(m: Vec<Vec<usize>>) -> Vec<String> {
    m.iter()
    .map(|row| row.iter()
        .map(|&x| if x == 1 { 'Q' } else { '.' }).collect::<String>())
    .collect()
}
