import java.util.Arrays;
import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Solution_2506 {

    public int similarPairs(String[] words){
        int count = 0;

        for (int i = 0; i < words.length; i++) {
            for (int j = i + 1; j < words.length; j++) {
                if (areSimilar(words[i], words[j])) {
                    count++;
                }
            }
            count -= 1;
        }

        return count/2;
    }

    boolean areSimilar(String s1, String s2) {
        // Convert both strings to character arrays and sort them
        char[] chars1 = s1.toCharArray();
        char[] chars2 = s2.toCharArray();
        Set<Character> set1 = new HashSet<>();
        Set<Character> set2 = new HashSet<>();

        Arrays.sort(chars1);
        Arrays.sort(chars2);
        for (char c : chars1)
            set1.add(c);
        for (char c : chars2)
            set2.add(c);

        return set1.equals(set2);
    }
}
