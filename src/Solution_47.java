import java.util.ArrayList;
import java.util.List;

public class Solution_47 {
    List<List<Integer>> res = new ArrayList<>();
    List<Integer> list = new ArrayList<>();
    public List<List<Integer>> permuteUnique(int[] nums) {
        for (int i = 0; i < nums.length; i++)
            helper(nums, i);

        return res;
    }
    public void helper(int[] nums, int index) {
        if (nums.length != 1) {
            int[] carry = new int[nums.length - 1];
            for (int i = 0, j = 0; i < carry.length; i++, j++)
                if (j != index)
                    carry[i] = nums[j];
                else
                    i--;

            list.add(nums[index]);
            for (int i = 0; i < carry.length; i++)
                helper(carry, i);
            list.remove(list.size() - 1);
        } else {
            list.add(nums[0]);
            if(!res.contains(List.copyOf(list)))
                res.add(List.copyOf(list));
            list.remove(list.size() - 1);
        }
    }
}
