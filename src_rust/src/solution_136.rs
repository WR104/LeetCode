pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut i = 0;
    while i < nums.len() - 1 {
        if nums[i] != nums[i+1] {
            return nums[i];
        } else {
            i += 2;
        }
    }       

    nums.last().unwrap()
}