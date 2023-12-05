class Solution:
    def computeArea(self, ax1: int, ay1: int, ax2: int, ay2: int, bx1: int, by1: int, bx2: int, by2: int) -> int:
        covered = 0
        if ax1 < bx1:
            left = ax1
            right = ax2
            need_1 = bx1
        else:
            left = bx1
            right =  bx2        
            need_1 = ax1
        
        if ay1 < by1:
            bottom = ay1
            top = ay2
            need_2 = by1
        else:
            bottom = by1
            top = by2
            need_2 = ay1
        for i in range(left, right+1):
            if i == need_1:
                for j in range(bottom, top+1):
                    if j == need_2:
                        covered = (min(ax2, bx2) - i) * (min(ay2, by2) - j)
                        break
        
        return (ax2-ax1) * (ay2-ay1) + (bx2-bx1) * (by2-by1) - covered

def test_helper(s):
    s = s.split(", ")
    result = [int(parts.split(" ")[2]) for parts in s]
    return result

def test_computeArea():
    tests = [
        (
            test_helper("ax1 = -3, ay1 = 0, ax2 = 3, ay2 = 4, bx1 = 0, by1 = -1, bx2 = 9, by2 = 2")
            , 45
        ),
        (
            test_helper("ax1 = -2, ay1 = -2, ax2 = 2, ay2 = 2, bx1 = -2, by1 = -2, bx2 = 2, by2 = 2")
            , 16
        )
    ]
    sol = Solution()
    for i, (case, expected) in enumerate(tests):
        result = sol.computeArea(case[0], case[1], case[2], case[3], case[4], case[5], case[6], case[7])
        assert result == expected, f"Test case {i + 1} failed: expected {expected}, got {result}"
        print(f"Test case {i + 1} passed!")

test_computeArea()