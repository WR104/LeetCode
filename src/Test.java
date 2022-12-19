import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Test {
	
public static void main(String[] args) {
	long startTime=System.currentTimeMillis();

	int[][] edges1 = {{11,12},{11,8},{4,2},{15,6},{6,11},{2,11},{12,16}};
	int[][] edges2 = {{1,2},{3,4}};
	int[][] edges3 = {{1,2},{1,3},{1,4}};

	int n1 = 17, n2 = 4, n3 = 4;

	List<List<Integer>> list1 = new ArrayList<>();
	for (int[] edge : edges1) {
		list1.add(Arrays.asList(edge[0], edge[1]));
	}
	List<List<Integer>> list2 = new ArrayList<>();
	for (int[] edge : edges2) {
		list2.add(Arrays.asList(edge[0], edge[1]));
	}
	List<List<Integer>> list3 = new ArrayList<>();
	for (int[] edge : edges3) {
		list3.add(Arrays.asList(edge[0], edge[1]));
	}

	System.out.println(new Solution_2508().isPossible(n1, list1));  // Outputs: true
//	System.out.println(new Solution_2508().isPossible(n2, list2));  // Outputs: true
//	System.out.println(new Solution_2508().isPossible(n3, list3));  // Outputs: false

	long endTime=System.currentTimeMillis(); 
	System.out.println("Running time: "+(endTime-startTime)+"ms");

}
	
}

