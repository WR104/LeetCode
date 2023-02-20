pub fn max_sub_array(nums: Vec<i32>) -> i32 {        
    let mut max = nums[0];
    let mut curr = max;

    for i in 1..nums.len() {
        if curr < 0 {
            curr = 0;
        }
        curr += nums[i];
        max = std::cmp::max(curr,max);
    }

    max
}