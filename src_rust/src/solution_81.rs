pub fn search(nums: Vec<i32>, target: i32) -> bool {
	let (mut left, mut right) = (0, (nums.len() - 1) as i32);
	while left <= right {
		let mid = (left + right) / 2;

		if nums[mid as usize] == target {
			return true
		} else if nums[mid as usize] < nums[right as usize] {
			if nums[mid as usize] < target && target <= nums[right as usize] {
				left = mid + 1;
			} else {
				right = mid - 1;
			}
		} else if nums[mid as usize] > nums[right as usize] {
			if nums[left as usize] <= target && target < nums[mid as usize] {
				right = mid - 1;
			} else {
				left = mid + 1;
			}
		} else {
			right -= 1;
		}
	}

	false
}