#[derive(Debug, Eq, PartialEq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
    if head.is_none() { return true }
    if head.as_ref().unwrap().next.is_none() { return true }

    // get the length
    let mut count = 0;
    let mut curr = &head;
    while let Some(n) = curr.as_ref() {
        count += 1;
        curr = &n.next;
    }
    println!("count: {count}");

    // reverse the first half
    let mut tail = &mut None;
    for _ in 0..count/2  {
        std::mem::swap(&mut head.as_mut().unwrap().next, tail);
        std::mem::swap(&mut head, &mut tail);
    }
    println!("reversed first half: {:?}", head);

    // extra movement if odd lengethed
    if count%2 == 1 {
        head = head.unwrap().next;
    }

    println!("head: {:?}\ntail: {:?}", head, tail);
    println!("result: {}\n\n", head == *tail);
    return head == *tail;
}


#[test]
fn test() {
    let n = Some(Box::new(ListNode::new(1)));
    let n = Some(Box::new(ListNode {val:2, next: n}));
    let n = Some(Box::new(ListNode {val:2, next: n}));
    let n = Some(Box::new(ListNode {val:1, next: n}));
    assert_eq!(true, is_palindrome(n));

    let n = Some(Box::new(ListNode::new(1)));
    let n = Some(Box::new(ListNode {val:2, next: n}));
    let n = Some(Box::new(ListNode {val:3, next: n}));
    let n = Some(Box::new(ListNode {val:2, next: n}));
    let n = Some(Box::new(ListNode {val:1, next: n}));
    assert_eq!(true, is_palindrome(n));

    let n = Some(Box::new(ListNode::new(1)));
    let n = Some(Box::new(ListNode {val:3, next: n}));
    let n = Some(Box::new(ListNode {val:2, next: n}));
    let n = Some(Box::new(ListNode {val:1, next: n}));
    assert_eq!(false, is_palindrome(n));

    let n = Some(Box::new(ListNode::new(1)));
    let n = Some(Box::new(ListNode {val:2, next: n}));
    let n = Some(Box::new(ListNode {val:3, next: n}));
    let n = Some(Box::new(ListNode {val:4, next: n}));
    let n = Some(Box::new(ListNode {val:1, next: n}));
    assert_eq!(false, is_palindrome(n));

    let n = Some(Box::new(ListNode::new(1)));
    let n = Some(Box::new(ListNode {val:2, next: n}));
    assert_eq!(false, is_palindrome(n));

    let n = Some(Box::new(ListNode::new(1)));
    assert_eq!(true, is_palindrome(n));
}
