pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn subsets_helper(nums: &Vec<i32>, index: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        result.push(current.clone());
        for i in index..nums.len() {
            if i > index && nums[i] == nums[i-1] {
                continue;
            }
            current.push(nums[i]);
            subsets_helper(nums, i + 1, current, result);
            current.pop();
        }
    }
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut nums = nums;
    nums.sort();
    subsets_helper(&nums, 0, &mut current, &mut result);
    result          
}