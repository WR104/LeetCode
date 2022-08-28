import java.util.Arrays;

public class Solution_35 {
    public int searchInser(int[] nums,int target){
        int index= Arrays.binarySearch(nums,target);
        if(index >= 0)
            return index;
        for(int i=0;i<nums.length;i++)
            if(nums[i] > target)
                return i;

        return nums.length;
    }
}
