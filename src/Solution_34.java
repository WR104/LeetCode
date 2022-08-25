import java.lang.reflect.Array;
import java.util.ArrayList;
import java.util.Arrays;

public class Solution_34 {
    public int[] searchRange(int[] nums, int target) {
        int start = helper(nums,target,0,nums.length-1);
        if(start == -1)
            return new int[] {-1,-1};

        return helper2(nums,target,start,start);
    }

    public int helper(int[] nums,int target,int start, int end){
        if(start <= end){
            int mid = start + (end-start)/2;
            if(nums[mid] == target)
                return mid;
            if(nums[mid] > target)
                return helper(nums,target,start,mid-1);
            return helper(nums,target,mid+1,end);
        }
        return -1;
    }
    public int[] helper2(int[] nums,int target,int left, int right){
        int l = left, r = right;
        if(left-1>=0 && nums[left-1] == target)
            l -= 1;
        if(right+1<=nums.length-1 && nums[right+1] == target)
            r += 1;
        if(l == left && r == right)
            return new int[]{l,r};
        else
            return helper2(nums,target,l,r);
    }
}
