import java.util.HashSet;
import java.util.Set;
import java.util.stream.Stream;

public class Solution_36 {

    public boolean isValidSudoku(char[][] board) {
        int n = board.length;
        int[][] f = new int[2 * n][9];
        int[][][] g = new int[n / 3][n / 3][9];
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if (board[i][j] != '.') {
                    int idx = board[i][j] - '1';
                    if (f[i][idx] > 0 || f[n + j][idx] > 0 || g[i / 3][j / 3][idx] > 0)
                        return false;
                    else {
                        f[i][idx]++;
                        f[n + j][idx]++;
                        g[i / 3][j / 3][idx]++;
                    }
                }

        return true;
    }

    public boolean myIsValidSudoku(char[][] board) {
        int n = board.length;
        int[][] f = new int[2 * n][9];
        int[][][] g = new int[n / 3][n / 3][9];
        for (int i = 0; i < n; i++)
            for (int j = 0; j < n; j++)
                if (board[i][j] != '.') {
                    int idx = board[i][j] - '1';
                    if (f[i][idx]++ > 0 || f[n + j][idx]++ > 0 || g[i / 3][j / 3][idx]++ > 0)
                        return false;
                }

        return true;
    }


}
