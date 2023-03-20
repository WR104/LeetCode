use std::collections::HashMap;

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut freq = HashMap::new();
    let mut cur = &head;
    while let Some(node) = cur.as_ref() {
        *freq.entry(node.val).or_insert(0) += 1;
        cur = &node.next;
    }
    let mut dummy = ListNode { val: 0, next: None };
    let mut tail = &mut dummy;
    cur = &head;
    while let Some(node) = cur.as_ref() {
        if freq[&node.val] == 1 {
            tail.next = Some(Box::new(ListNode { val: node.val, next: None }));
            tail = tail.next.as_mut().unwrap();
        }
        cur = &node.next;
    }
    dummy.next
}