use std::collections::HashSet;

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn subsets_helper(nums: &Vec<i32>, index: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, set: &mut HashSet<Vec<i32>>) {
        if !set.contains(current) {
            result.push(current.clone());
            set.insert(current.clone());
        }
        if index == nums.len() {
            return;
        }
        current.push(nums[index]);
        subsets_helper(nums, index + 1, current, result, set);
        current.pop();
        subsets_helper(nums, index + 1, current, result, set);
    }

    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut set = HashSet::new();
    subsets_helper(&nums, 0, &mut current, &mut result, &mut set);
    result
}