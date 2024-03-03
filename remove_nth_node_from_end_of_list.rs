#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    
    if head.as_ref().unwrap().next.is_none() { return None }

    let mut cnt = 0;
    let mut node = head.as_ref();
    while let Some(n) = node {
        cnt += 1;
        node = n.next.as_ref();
    }
    if cnt == n { return head.unwrap().next }

    cnt = cnt - n;
    let mut node = head.as_mut();
    while cnt > 1 {
        cnt -= 1;
        node = node.unwrap().next.as_mut();
    }

    node.unwrap().next = node.as_mut().unwrap().next.as_mut().unwrap().next.take();
    return head;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let n = ListNode::new(5);
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        println!("{:?}", remove_nth_from_end(Some(Box::new(n)), 5));

        let n = ListNode::new(5);
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        println!("{:?}", remove_nth_from_end(Some(Box::new(n)), 4));

        let n = ListNode::new(5);
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        println!("{:?}", remove_nth_from_end(Some(Box::new(n)), 3));

        let n = ListNode::new(5);
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        println!("{:?}", remove_nth_from_end(Some(Box::new(n)), 2));

        let n = ListNode::new(5);
        let n = ListNode { val: 4, next: Some(Box::new(n)) };
        let n = ListNode { val: 3, next: Some(Box::new(n)) };
        let n = ListNode { val: 2, next: Some(Box::new(n)) };
        let n = ListNode { val: 1, next: Some(Box::new(n)) };
        println!("{:?}", remove_nth_from_end(Some(Box::new(n)), 1));
    }

    #[test]
    fn test2() {
        let n = Some(Box::new(ListNode::new(1)));
        remove_nth_from_end(n, 1);
    }
}
