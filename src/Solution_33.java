public class Solution_33 {
    public int search(int[] nums, int target) {
        if(target >= nums[0]){
            for(int i=0;i<nums.length;i++)
                if(target == nums[i])
                    return i;
        }
        else{
            for(int i=nums.length-1;i>=0;i--)
                if(target == nums[i])
                    return i;
        }
        return -1;
    }
}
