use std::cmp::Ordering::{Equal, Greater, Less};
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len() as i32;
    let n = matrix[0].len() as i32;
    let mut left: i32 = 0;
    let mut right: i32 = m * n - 1;
    
    while left <= right {
        let mid = (left + right + 1) / 2;
        let (row, col) = ((mid / n) as usize, (mid % n) as usize);
        match target.cmp(&matrix[row][col]){
            Equal => return true,
            Less => left = mid + 1,
            Greater => right = mid -1,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_74() {
        assert_eq!(search_matrix(vec![vec![1]], 0), false);
        assert_eq!(search_matrix(vec![vec![1],vec![3]], 3), true);
    }
}