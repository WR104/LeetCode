pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut row = vec![1];
    for i in 1..=row_index {
        for j in (1..i).rev() {
            row[j as usize] += row[j as usize - 1];
        }
        row.push(1);
    }
    row
}