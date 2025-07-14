struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    
    let mut curr = head.as_ref();
    let mut ans = 0;
    while let Some(node) = curr {
        ans <<= 1;
        if node.val == 1 {
            ans ^= 1;
        }
        curr = node.next.as_ref();
    }
    return ans;
}
