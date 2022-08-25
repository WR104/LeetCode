import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Test {
	
public static void main(String[] args) {
	long startTime=System.currentTimeMillis(); 
	
	int[] nums = {};

	System.out.println(Arrays.toString(new Solution_34().searchRange(nums,8)));

	long endTime=System.currentTimeMillis(); 
	System.out.println("Running time: "+(endTime-startTime)+"ms");

}
	
}

