pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut triangle = triangle;
    for i in 1..triangle.len() {
        for j in 0..triangle[i].len() {
            let left = if j > 0 { triangle[i - 1][j - 1] } else { i32::MAX };
            let up = if j < triangle[i - 1].len() { triangle[i - 1][j] } else { i32::MAX };
            triangle[i][j] += left.min(up);
        }
    }
    *triangle.last().unwrap().iter().min().unwrap()
}