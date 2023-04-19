pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    nums.rotate_right(k as usize % nums.len());
}
