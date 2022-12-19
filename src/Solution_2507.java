import java.util.ArrayList;
import java.util.List;

public class Solution_2507 {
    public int smallestValue(int n){
        if(n == 4)
            return n;
        var res = 0;
        var list = getPrimeFactors(n);
        while(list.size() > 1) {
            res = list
                    .stream()
                    .mapToInt(Integer::intValue)
                    .sum();
            list = getPrimeFactors(res);
        }
        return list.get(0);
    }

    public List<Integer> getPrimeFactors(int n){
        List<Integer> factors = new ArrayList<>();

        if (n <= 1)
            return factors;

        for (int i = 2; i <= Math.sqrt(n); i++) {
            while (n % i == 0) {
                factors.add(i);
                n /= i;
            }
        }

        if (n > 1)
            factors.add(n);

        return factors;
    }

}
