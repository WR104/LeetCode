public class Solution_27 {
    public int removeElement(int[] nums, int val) {
        int start = 0;
        int end = 0;
        int n = nums.length;
        while (end < n) {
            while(end < n && nums[end] == val)
                end++;
            if(end == n)
                break;
            nums[start++] = nums[end++];
        }
        return start;
    }
}
