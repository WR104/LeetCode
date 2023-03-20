pub fn sort_colors(nums: &mut Vec<i32>) {
    let n = nums.len();
    let mut index_0 = 0;
    let mut index_1 = 0;
    for i in 0..n {
        match nums[i] {
            0 => {
                nums.swap(i, index_0);
                if index_0 != index_1 {
                    nums.swap(i, index_1);
                }
                index_0 += 1;
                index_1 += 1;
            },
            1 => {
                nums.swap(i, index_1);
                index_1 += 1;
            },
            _ => {},
        }
    }
}