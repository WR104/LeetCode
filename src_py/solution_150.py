from typing import List

def is_digit(string):
    if string.startswith('-'):
        return string[1:].isdigit()
    return string.isdigit()

class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        result = []
        for e in tokens:
            if is_digit(e):
                result.append(int(e))
            else:
                c = result.pop()
                d = result.pop()
                if e == "+":
                    r = d+c
                elif e == "-":
                    r = d-c
                elif e == "*":
                    r = d*c
                elif e == "/":
                    r = int(d/c)
                result.append(r)

        return result[0]
