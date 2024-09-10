#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 { return b }
    if b == 0 { return a }
    if a == b { return a }
    if a > b { return gcd(a-b, b) }
    return gcd(b-a, a);
}

pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    let mut curr = &mut head;
    while let Some(node) = curr.as_mut() {
        
        if node.next.is_none() { return head }
        
        // there are 2 nodes. A node needs to be inserted.
        // currently:  v -> u -> ...
        // after: v -> gcd(v,u) -> u -> ...
        let v = node.val;
        let u = node.next.as_ref().unwrap().val;
        let added_node = Some(Box::new(ListNode { val: gcd(v,u), next: node.next.take() }));
        node.next = added_node;
        curr = &mut node.next.as_mut().unwrap().next;
    }

    return head;
}

fn _make_ll(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut curr = None;
    for v in values.into_iter().rev() {
        curr = Some(Box::new(ListNode {val: v, next: curr}));
    }
    return curr;
}

#[test]
fn tests() {
    
    let head = _make_ll(vec![18,6,10,3]);
    let ans = _make_ll(vec![18,6,6,2,10,1,3]);
    assert_eq!(ans, insert_greatest_common_divisors(head));

    let head = _make_ll(vec![7]);
    let ans = _make_ll(vec![7]);
    assert_eq!(ans, insert_greatest_common_divisors(head));

    let head = _make_ll(vec![7,7,7]);
    let ans = _make_ll(vec![7,7,7,7,7]);
    assert_eq!(ans, insert_greatest_common_divisors(head));

    let head = _make_ll(vec![7,7,7,7]);
    let ans = _make_ll(vec![7,7,7,7,7,7,7]);
    assert_eq!(ans, insert_greatest_common_divisors(head));

    let head = _make_ll(vec![7,7,7]);
    let ans = _make_ll(vec![7,7,7,7,7]);
    assert_eq!(ans, insert_greatest_common_divisors(head));
}
