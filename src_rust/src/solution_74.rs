/*
 * @lc app=leetcode id=74 lang=rust
 *
 * [74] Search a 2D Matrix
 */

// @lc code=start

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut left = 0;
        let mut right = matrix.len() - 1;
        while left <= right {
            let mid  = (right + left) / 2;
            if matrix[mid][0] <= target{
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        let row = right;
        left = 0;
        right = matrix[row].len() - 1;
        while left <= right {
            let mid = (right + left) / 2;
            if matrix[row][mid] == target{
                return true;
            } else if matrix[row][mid] <= target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        false
    }
}

pub struct Solution{}
// @lc code=end

