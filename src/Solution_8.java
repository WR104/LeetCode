public class Solution_8 {
	public int bestSolution(String s) {
		if(s.isEmpty())
			return 0;
		s = s.trim();
		int sign = 1, start = 0;
		if(s.charAt(start) == '+' || s.charAt(start) == '-')
			sign = s.charAt(start++) == '+' ? 1: -1;
		
		long res = 0;
		
		while(start < s.length()) {
			if(!Character.isDigit(s.charAt(start)))
				return sign * (int)res;
			
			int num = s.charAt(start++) - '0';
			res = res * 10 + num;
			
			if(res > Integer.MAX_VALUE)
				return sign == 1? Integer.MAX_VALUE: Integer.MIN_VALUE;
		}
		return sign * (int) res;
		
	}
	public int myAtoi(String s) {
		s = s.trim()+" ";
		int stop = 0;
		int ans;
		boolean sign = true;
		if(s.charAt(0) == '-') {
			sign = false;
			stop += 1;
		}
		if(s.charAt(0) == '+')
			s = s.substring(1,s.length());
		
		
		for(int i = (sign?0:1); i < s.length()-1; i++) {
			if(Character.isDigit(s.charAt(i))) {
				stop +=1;
			}
			else
				break;
		}
		
		if(s.substring((sign?0:1),stop).isEmpty())
			ans = 0;
		else {
			if(outOfRange(s.substring((sign?0:1),stop)))
				return sign?Integer.MAX_VALUE:Integer.MIN_VALUE;
			else
				ans = Integer.parseInt(s.substring((sign?0:1),stop));
		}
			
		return sign?ans:-1*ans;
	}
	public boolean outOfRange(String str) {
        int length = str.length();
        if(length == 0){
            return false;
        }
        int index = 0;
        int sum = 0; // ��¼�ۼӽ��
        while(index < length) {
            int digit = str.charAt(index) - '0';
            // ����ٶ�str�ǺϷ����ַ���������Ҫ����digit�Ϸ����ж�

            if(Integer.MAX_VALUE/10 < sum || (Integer.MAX_VALUE/10 == sum && Integer.MAX_VALUE % 10 < digit)) {
                // ˵�����
                return true;
            }
            // ˵����û�����
            sum = sum * 10 + digit;
            index ++;
        }
        return false;
    }
	
}
