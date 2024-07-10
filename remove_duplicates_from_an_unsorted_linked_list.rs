#[derive(Debug, Eq, PartialEq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    fn from(vals: Vec<i32>) -> Option<Box<Self>> {
        let mut head = None;
        for v in vals.iter().rev() {
            let n = Some(Box::new(ListNode{val: *v, next: head}));
            head = n;
        }
        return head;
    }
}


pub fn delete_duplicates_unsorted(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut counts = [0; 100001];
    
    // Get the # occurances of each val in the list
    let mut h = head.as_ref();
    while let Some(n) = h {
        counts[n.val as usize] += 1;
        h = n.next.as_ref();
    }

    // Only keep the non duplicate values
    let mut dummy = ListNode::new(-1);
    let mut tail = &mut dummy.next;
    while let Some(mut n) = head.take() {
        head = n.next.take();
        if counts[n.val as usize] < 2 {
            *tail = Some(n);
            tail = &mut tail.as_mut().unwrap().next;
        }
    }
    return dummy.next;
}

#[test]
fn tests() {
    let v = vec![1,2,3,2];
    let l = ListNode::from(v);
    let ans = ListNode::from(vec![1,3]);
    assert_eq!(ans, delete_duplicates_unsorted(l));

    let v = vec![2,1,1,2];
    let l = ListNode::from(v);
    let ans = ListNode::from(vec![]);
    assert_eq!(ans, delete_duplicates_unsorted(l));

    let v = vec![3,2,2,1,3,2,4];
    let l = ListNode::from(v);
    let ans = ListNode::from(vec![1,4]);
    assert_eq!(ans, delete_duplicates_unsorted(l));
}
