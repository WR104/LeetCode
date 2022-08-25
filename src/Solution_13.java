
public class Solution_13 {
	final static int[] val = { 1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1 };
	final static String[] rom = { "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I" };

	int romanToInt(String s) {
		int n = 0, m = 1;
		int result = 0;

		while (s.length() > 0) {

			if (m < rom.length && s.indexOf(rom[m]) != -1) {
				result += val[m];
				s = s.substring(2, s.length());
			}
			m += 2;

			while (s.contains(rom[n])) {
				result += val[n];
				s = s.substring(1);
			}
			n += 2;

		}

		return result;
	}
}
