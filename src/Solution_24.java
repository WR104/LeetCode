class Solution_24 {
    public ListNode swapPairs(ListNode node) {
        if(node == null || node.next == null)
            return node;
        
        ListNode nextNode = node.next;
        node.next = swapPairs(nextNode.next);
        nextNode.next = node;

        return nextNode;

    }
}