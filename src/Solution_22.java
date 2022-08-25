import java.util.*;
import java.util.stream.Collectors;

public class Solution_22 {
	List<String> list = new ArrayList<String>();

	public List<String> generateParenthesis(int n) {

		fun("()",n);
		return list.stream().distinct().collect(Collectors.toList());
	}
	
	public void fun (String s, int n) {
		if(n == 1) {
			list.add(s);
			return;
		}
		
		fun("()"+s,n-1);
		for(int i=1;i<s.length();i++) {
			fun(s.substring(0,i)+"()"+s.substring(i),n-1);
		}
		fun(s+"()",n-1);
		
	}
}
