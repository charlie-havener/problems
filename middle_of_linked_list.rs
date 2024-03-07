struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None, }
    }
}

pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
     

    let mut len = 0;
    let mut h = head.as_ref();

    while let Some(node) = h {
     len += 1;
     h = node.next.as_ref();
    }

    let m = len/2;

    let mut h = head.as_mut();
    for _ in 1..m {
     h = h.unwrap().next.as_mut();
    }

    return h.unwrap().next.take();

}
