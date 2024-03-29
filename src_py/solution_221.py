from typing import List


class Solution:
    def maximalSquare(self, matrix: List[List[str]]) -> int:   
        dp = [[int(element) for element in row] for row in matrix]
        result = 0
        for row in range(len(dp)):
            for col in range(len(dp[0])):
                if matrix[row][col] == "1":
                    if row > 0 and col > 0:
                        dp[row][col] = min(dp[row-1][col-1], dp[row-1][col],\
                                           dp[row][col-1]) + 1
                    result = max(result, dp[row][col])
        return result**2


def test_maximalSquare():
    # Test cases
    tests = [
        ([
            ["1","0","1","0","0"],
            ["1","0","1","1","1"],
            ["1","1","1","1","1"],
            ["1","0","0","1","0"]
        ], 4),
        ([
            ["0","0","0","0"],
            ["0","0","0","0"],
            ["0","0","0","0"]
        ], 0),
        ([
            ["0","1"],
            ["1","0"]
        ], 1),
        ([
            ["1","1"],
            ["1","1"]
        ], 4),
        ([["1"],["0"],["1"],["1"],["1"],["1"],["0"]]
         ,1),
         (
             [["1","0"],["0","1"],["0","1"],["0","1"],["1","1"],["0","0"],["0","1"]]
        , 1),
        ([["0","0","0","1"],["1","1","0","1"],["1","1","1","1"],["0","1","1","1"],["0","1","1","1"]]
         , 9),
        ([["0","0","1","0"],["1","1","1","1"],["1","1","1","1"],["1","1","1","0"],["1","1","0","0"],["1","1","1","1"],["1","1","1","0"]]
          ,9),
        ([["1","0","1","1","0","1"],["1","1","1","1","1","1"],["0","1","1","0","1","1"],["1","1","1","0","1","0"],["0","1","1","1","1","1"],["1","1","0","1","1","1"]]
           ,4),
        ([["1","1","1","1","0"],["1","1","1","1","0"],["1","1","1","1","1"],["1","1","1","1","1"],["0","0","1","1","1"]]
         , 16),
        ([["0","0","0","1","0","1","1","1"],["0","1","1","0","0","1","0","1"],["1","0","1","1","1","1","0","1"],["0","0","0","1","0","0","0","0"],["0","0","1","0","0","0","1","0"],["1","1","1","0","0","1","1","1"],["1","0","0","1","1","0","0","1"],["0","1","0","0","1","1","0","0"],["1","0","0","1","0","0","0","0"]]
         , 1)
    ]

    sol = Solution()
    for i, (matrix, expected) in enumerate(tests):
        result = sol.maximalSquare(matrix)
        assert result == expected, f"Test case {i + 1} failed: expected {expected}, got {result}"
        print(f"Test case {i + 1} passed!")

test_maximalSquare()