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

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    // find the length
    let mut length = 0;
    let mut curr = head.as_mut();
    while let Some(node) = curr {
        length += 1;
        curr = node.next.as_mut()
    }

    if length <= 2 { return; }

    // get to the start of the second half of the list
    let mut curr = head.as_mut();
    for _ in 0..length/2 {
        curr = curr.unwrap().next.as_mut();
    }

    let mut half = curr.unwrap().next.take();

    // reverse the second half of the list
    let mut reversed = ListNode::new(0);
    while let Some(mut node) = half.take() {
        half = node.next.take();
        node.next = reversed.next.take();
        reversed.next = Some(node);
    }

    let mut tail = &mut head.as_mut().unwrap().next;
    while tail.is_some() && reversed.next.is_some() {
        let mut r = reversed.next.take().unwrap();
        reversed.next = r.next.take();

        r.next = tail.take();
        *tail = Some(r);
        tail = &mut tail.as_mut().unwrap().next;
        if let Some(node) = tail {
            tail = &mut node.next;
        }
    }
}

#[test]
fn tests() {
    let n = Some(Box::new(ListNode::new(4)));
    let n = Some(Box::new(ListNode { val: 3, next: n }));
    let n = Some(Box::new(ListNode { val: 2, next: n }));
    let mut n = Some(Box::new(ListNode { val: 1, next: n }));
    
    let m = Some(Box::new(ListNode::new(3)));
    let m = Some(Box::new(ListNode { val: 2, next: m }));
    let m = Some(Box::new(ListNode { val: 4, next: m }));
    let m = Some(Box::new(ListNode { val: 1, next: m }));

    reorder_list(&mut n);
    assert_eq!(m, n);
}

#[test]
fn test2() {
    let n = Some(Box::new(ListNode::new(5)));
    let n = Some(Box::new(ListNode { val: 4, next: n }));
    let n = Some(Box::new(ListNode { val: 3, next: n }));
    let n = Some(Box::new(ListNode { val: 2, next: n }));
    let mut n = Some(Box::new(ListNode { val: 1, next: n }));
    
    let m = Some(Box::new(ListNode::new(3)));
    let m = Some(Box::new(ListNode { val: 4, next: m }));
    let m = Some(Box::new(ListNode { val: 2, next: m }));
    let m = Some(Box::new(ListNode { val: 5, next: m }));
    let m = Some(Box::new(ListNode { val: 1, next: m }));

    reorder_list(&mut n);
    assert_eq!(m, n);
}
