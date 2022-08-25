import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Solution_17 {
	int[] nums = {2,3,4,5,6,7,8,9};
	String[] ch = {"abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"};
	Map<Integer, String> map = IntStream.range(0, nums.length).boxed()
			.collect(Collectors.toMap(i->nums[i], i->ch[i]));
	
	public List<String> letterCombinations(String digit) {
        List<String> result = new ArrayList<String>();
        if(digit.length() == 0)
			return result;
        int[] index = new int[digit.length()];
        for(int i=0 ; i<index.length; i++)
        	index[i] = 0;
        final int[] digits = new int[digit.length()];
        for(int i = 0; i<digits.length; i++)
        	digits[i] = digit.charAt(i) - '0';
        
        while(index[0] < map.get(digits[0]).length()) {
        	result.add(merge(index,digits));
        	index[index.length-1]++;
        	carry(index,digits);
        }
        return result;
    }
	
	public String merge(int index[], int digits[]) {
		String res = "";
		for(int i=0; i<digits.length; i++) {
			res += map.get(digits[i]).charAt(index[i]);
		}
		return res;
	}
	
	public void carry(int index[], int digits[]) {
		for(int i=index.length-1; i>0; i--) {
			if(index[i] >= map.get(digits[i]).length()) {
				index[i] = 0;
				index[i-1] ++;
			}
			else
				break;
		}
	}
}
