import java.util.*;

public class Solution_30 {
    public List<Integer> findSubstring(String s, String[] words) {
        List<Integer> res = new ArrayList<Integer>();
        int n=0;
        for(int i=0; i<words.length;i++){
            n += words[i].length();
        }

        for(int i=0; i<s.length()-n+1; i++){
            String temps = s.substring(i,i+n);

            for(int j=0; j<words.length; j++){
            }
        }
        return res;
    }
}
