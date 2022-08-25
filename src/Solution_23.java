import java.util.*;
public class Solution_23 {
	public ListNode mergeKLists(ListNode[] lists) {

        List<Integer> cur = new ArrayList<Integer>();        	
        if(lists.length == 0)
        	return null;
        
        for(int i=0;i<lists.length;i++) {
        	while(lists[i] != null) {
        		cur.add(lists[i].val);
        		lists[i] = lists[i].next;
        	}
        }
        
        if(cur.size() == 0)
        	return null;
        Collections.sort(cur);
        ListNode head;
        ListNode nextNode = new ListNode(cur.get(0));
        head = nextNode;
        for(int i=1;i<cur.size();i++) {
        	ListNode node = new ListNode(cur.get(i));
        	nextNode.next = node;
        	nextNode = nextNode.next;
        }
        return head;
    }
}
