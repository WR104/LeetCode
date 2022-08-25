import java.util.Arrays;

public class Solution_4 {
	public double findMedianSortedArrays(int [] nums1, int[] nums2) {
		int[] nums3 = new int[nums1.length+nums2.length];
		int i=0, j=0, k=0;
		while(k < nums3.length) {
			if(i < nums1.length) {
				if(j == nums2.length)
					nums3[k++] = nums1[i++];
				else
					nums3[k++] = nums1[i] <= nums2[j] ? nums1[i++] : nums2[j++];
			}
			else {
				nums3[k++] = nums2[j++];
			}
		}		
		return nums3.length%2 == 1 ? nums3[nums3.length/2]/1.0 : (nums3[nums3.length/2]+nums3[nums3.length/2-1])/2.0;
	}
	
	public double anotherSolution(int[] nums1, int[] nums2) {
		int[] nums = new int[nums1.length + nums2.length];
		for(int i=0;i<nums1.length;i++)
			nums[i]=nums1[i];
		for(int j=0;j<nums2.length;j++)
			nums[nums1.length+j	]=nums[j];
		
		Arrays.sort(nums);
		return nums.length%2 == 1 ? nums[nums.length/2]/1.0 : (nums[nums.length/2]+nums[nums.length/2-1])/2.0;
	}

}
