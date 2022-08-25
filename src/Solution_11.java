import java.util.Arrays;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.IntStream;public class Solution_11 {
	public int maxArea(int[] height) {
        
		int result = 0;
        if(height.length < 2)
        	return 0;
        
		for(int i = 0; i < height.length; i++) {
			for(int j = height.length - 1; j > i; j--)
				if(height[i] <= height[j]) {
					result = Math.max(result, height[i]*(j-i));
					break;
				}else {
					result = Math.max(result, height[j]*(j-i));
					//break;
				}
		}

		return result;
    }
}
