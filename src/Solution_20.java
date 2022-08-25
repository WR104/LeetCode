import java.util.*;
public class Solution_20 {
public boolean isValid(String s) {
        
	Deque<Character> leftChars = new LinkedList<>();
    
    if(s.length() <= 1) {
        return false;
    }
    
    for(Character ch : s.toCharArray()) {
        if(ch == '(' || ch == '{' || ch == '[') 
            leftChars.addFirst(ch);
        else {
            if(leftChars.isEmpty() || 
               (ch == ')' && leftChars.peekFirst() != '(' ) ||
               (ch == '}' && leftChars.peekFirst() != '{' ) || 
               (ch == ']' && leftChars.peekFirst() != '[' )
              )
                return false;
            
            leftChars.removeFirst();
        }
    }
    
    return leftChars.isEmpty();
}
}
