import java.util.*;
import java.util.stream.Collectors;

public class Solution_2508 {    //failed
    public boolean isPossible(int n, List<List<Integer>> edges) {
        int[] degree = new int[n + 1];  // Degree of each node
        Set<Integer> oddDegreeNodes = new HashSet<>();  // Nodes with odd degree

        // Increment the degree of each node and add odd degree nodes to the list
        for (List<Integer> edge : edges) {
            int a = edge.get(0), b = edge.get(1);
            degree[a]++;
            degree[b]++;
            if(degree[a] % 2 == 1){
                oddDegreeNodes.add(a);
            } else {
                oddDegreeNodes.remove(a);
            }
            if(degree[b] % 2 == 1){
                oddDegreeNodes.add(b);
            } else {
                oddDegreeNodes.remove(b);
            }
        }
        if(oddDegreeNodes.size() == 0)
            return true;
        if(oddDegreeNodes.size() > 4)
            return false;

        List<List<Integer>> pairs = new ArrayList<>();
        for (int i = 1; i <= n; i++) {
            for (int j = i + 1; j <= n; j++) {
                if(!edges.contains(Arrays.asList(i,j)) && !edges.contains(Arrays.asList(j,i)))
                    pairs.add(Arrays.asList(i, j));
            }
        }

        for(int i=0;i<pairs.size();i++){
            int a = pairs.get(i).get(0), b = pairs.get(i).get(1);
            Set<Integer> temp = new HashSet<>(oddDegreeNodes);
            if(temp.contains(a))
                temp.remove(a);
            else
                temp.add(a);
            if(temp.contains(b))
                temp.remove(b);
            else
                temp.remove(b);
            if(temp.size() == 0) {
                return true;
            }
            for(int j=0;j<pairs.size();j++){
                if(i != j) {
                    int c = pairs.get(j).get(0), d = pairs.get(j).get(1);
                    if(temp.contains(c) && temp.contains(d))
                        return true;
                }
            }
        }

        return false;
    }
}
