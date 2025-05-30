
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}

pub fn delete_nodes(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = ListNode { val: -1, next: head };
    let mut curr = &mut dummy;
    loop {
        for _ in 0..m {
            if curr.next.is_none() { return dummy.next }
            curr = curr.next.as_mut().unwrap();
        }
        for _ in 0..n {
            if curr.next.is_none() { return dummy.next }
            curr.next = curr.next.as_mut().unwrap().next.take();
        }
    }
}
