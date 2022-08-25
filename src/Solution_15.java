import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Solution_15 {
	public List<List<Integer>> threeSum(int[] nums){
		List<List<Integer>> list = new ArrayList<List<Integer>>();
		Set<List<Integer>> set = new HashSet<List<Integer>>(list);
		Arrays.sort(nums);
		if(nums.length < 3)
			return list;
		
		for(int i=0; i<nums.length-2; i++) {
			if(nums[i] > 0)
				break;
			if(i>0 && nums[i] == nums[i-1])
				continue;
			for(int j=i+1; j<nums.length-1; j++) {
				if(j>i+2 && nums[j] == nums[j-1] && nums[j-1] == nums[j-2])
					continue;
				for(int k=j+1; k<nums.length; k++) {
					if(nums[i] + nums [j] + nums[k] == 0) {
						List<Integer> temp = new ArrayList<Integer>();
						temp.add(nums[i]);
						temp.add(nums[j]);
						temp.add(nums[k]);
						set.add(temp);
						
					}
											
				}
			}
		}
		list.addAll(set);
		return list;
	}

}
