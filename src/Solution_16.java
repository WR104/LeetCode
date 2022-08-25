import java.util.Arrays;

public class Solution_16 {
	public int threeSumClosest(int[] nums, int target) {
		if (nums.length == 3)
			return nums[0] + nums[1] + nums[2];
		Arrays.sort(nums);
		int left = 0, right = nums.length - 1, mid = left + 1, min = Integer.MAX_VALUE, res = 0;
		while (left <= nums.length - 3) {
			right = nums.length - 1;
			mid = left + 1;
			while (mid < right) {
				int diff = nums[left] + nums[right] + nums[mid];

				if (diff == target)
					return target;
				else if (diff > target)
					right--;
				else
					mid++;

				int localMin;

				if (diff < target) {
					localMin = target - diff;

				} else {
					localMin = diff - target;

				}
				if (localMin < min) {
					res = diff;
					min = localMin;
				}
			}
			left++;
		}
		return res;
	}
	 public int mythreeSumClosest(int[] nums, int target) {
	        int result = nums[0] + nums[1] +nums[2];
	        int count = 0;
	        if(nums.length == 3)
	        	return result;
	        for(int i=0;i<nums.length-2;i++){
	            for(int j=i+1;j<nums.length-1;j++){
	                for(int k=j+1;k<nums.length;k++){
	                    int temp = nums[i] + nums[j] + nums[k];
	                    if(Math.abs(temp - target) <= Math.abs(result - target))
	                        result = temp;    
	                    count++;
	                }
	            }
	        } 
		        return result;
	 }
}