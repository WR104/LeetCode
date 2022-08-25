import java.util.*;
public class Solution_31 {
    public void nextPermutation(int[] nums) {
        int n = nums.length;
        int index = -1;
        int cur = -1;
        int min = Integer.MAX_VALUE;
        for (int i = n - 1; i >= 0; i--) {
            if (i == 0) {
                reverse(nums);
                break;
            } else if (nums[i - 1] < nums[i]) {
                index = i - 1;
                for (int k = n - 1; k > index; k--)
                    if (nums[k] > nums[index] && min > nums[k] - nums[index]) {
                        min = nums[k] - nums[index];
                        cur = k;
                    }
                int temp = nums[cur];
                nums[cur] = nums[index];
                nums[index] = temp;

                Arrays.sort(nums, index + 1, n);
                break;
            }
        }
    }


    void reverse(int[] validData){
        for(int i = 0; i < validData.length / 2; i++)
        {
            int temp = validData[i];
            validData[i] = validData[validData.length - i - 1];
            validData[validData.length - i - 1] = temp;
        }
    }
}
