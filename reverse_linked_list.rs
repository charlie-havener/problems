struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self {val, next: None }
    }
}

pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    
    let tail = &mut None;
    while let Some(curr) = head.as_mut() {
        std::mem::swap(&mut curr.next, tail);
        std::mem::swap(&mut head, tail);
    }
    std::mem::swap(&mut head, tail);

    return head;
}
