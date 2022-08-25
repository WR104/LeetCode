import java.util.Arrays;

public class Solution_12 {
	char[] basic = {'I','X','C','M',};
	char[] upper = {'V','L','D'};
public String intToRoman(int num) {
		int[] nums = new int[(int)(Math.log10(num)+1)];
		String s = String.valueOf(num);
		String result = "";
		for(int j=0,i=nums.length-1; i>=0; i--,j++) 
			result = change(s.charAt(j)-'0',j) + result;
		
		
		return result;
    }

public String change(int num, int digit) {
	String result = "";
	if(num < 4) {
		for(int i=0; i<num; i++)
			result += basic[digit];
	}
	else if(num == 4)
		result += String.valueOf(basic[digit]) + upper[digit];
	else if(num < 9){
		result += upper[digit];
		for(int i=5; i<num; i++)
			result += basic[digit];
	}
	else if(num == 9)
		result += String.valueOf(basic[digit]) + basic[digit+1];
	return result;
		
}
}
