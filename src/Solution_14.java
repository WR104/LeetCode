
public class Solution_14 {
	public String longestCommonPrefix(String[] strs) {
		String result = "";
		String str = strs[0];
		if(strs.length == 1)
			return strs[0];
		
		outerloop:
		for(int i = 0; i<str.length(); i++) {
			for(int j = 1; j < strs.length; j++) {
				if(i >= strs[j].length() || str.charAt(i) != strs[j].charAt(i))
					break outerloop;
				if(j == strs.length-1)
					result = str.substring(0,i+1);

			}
		}
		return result;
	}
}
