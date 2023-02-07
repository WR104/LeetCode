pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let n = nums.len();
    for i in 0..n {
        while 0 < nums[i] && nums[i] <= n as i32 && nums[nums[i] as usize - 1] != nums[i] {
            let temp = nums[nums[i] as usize - 1];
            let temp_index = nums[i] as usize - 1;
            nums[temp_index] = nums[i];
            nums[i] = temp;
        }
    }
    for i in 0..n {
        if nums[i] != i as i32 + 1 {
            return i as i32 + 1;
        }
    }
    n as i32 + 1
}