pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut cur = 1;
    let mut res = 1;
    for i in 0..nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            continue;
        }
        if nums[i] + 1 == nums[i + 1] {
            cur += 1;
            res = res.max(cur);
        } else {
            cur  = 1;
        }
    }     
    res
}