pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut count = 1;
    let mut i = 0;
    while i < nums.len() - 1 {
        i += 1;
        match (nums[i], nums[i-1]) {
            (x, y) if x == y => {
                count += 1;
                if count > 2 {
                    let mut j = i;
                    while j < nums.len() && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                    nums.drain(i..j);
                    count = 1;
                }
            },
            _ => count = 1,
        }
    }
    nums.len() as i32
}