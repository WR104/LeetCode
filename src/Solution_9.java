class Solution_9 {
	public boolean isPalindrome(int x) {
		if(x >= 0) {
			String s = String.valueOf(x);
			String reverse = new StringBuffer(s).reverse().toString();
			if(s.equals(reverse))
				return true;
		}
		return false;
			
	}
}
