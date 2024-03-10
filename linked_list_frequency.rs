use std::collections::HashMap;

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None, }
    }
}

pub fn frequencies_of_elements(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut hm = HashMap::new();

    let mut node = &head;
    while let Some(n) = node.as_ref() {
        hm.entry(n.val).and_modify(|v| *v += 1).or_insert(1);
        node = &n.next;
    }

    let mut ans = Some(Box::new(ListNode::new(0)));
    let mut curr = &mut ans;

    for v in hm.values() {
        curr.as_mut().unwrap().next = Some(Box::new(ListNode::new(*v)));
        curr = &mut curr.as_mut().unwrap().next;
    }

    return ans.as_mut().unwrap().next.take();
}
