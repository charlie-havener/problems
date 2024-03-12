#[derive(Debug, Eq, PartialEq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self {val, next: None }
    }
}

use std::collections::HashMap;

pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() { return None }

    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;

    let mut hm = HashMap::new();
    hm.insert(0, dummy.as_ref());

    let mut sum = 0;
    let mut curr = dummy.next.as_ref();
    while let Some(n) = curr {
        sum += n.val;
        hm.insert(sum, n.as_ref());
        curr = n.next.as_ref();
    }
    
    let mut ans = Box::new(ListNode::new(0));
    sum = 0;
    let mut curr = Some(&mut ans);
    while let Some(n) = curr {
        sum += n.val;
        if let Some(mapped) = hm.get(&sum) {
            n.next = match mapped.next.as_ref() {
                Some(next) => Some(Box::new(ListNode::new(next.val))),
                None => None,
            }
        }
        curr = n.next.as_mut();
    }
    return ans.next;
}

fn to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut curr = &mut head;

    for n in nums {
        curr.as_mut().unwrap().next = Some(Box::new(ListNode::new(n)));
        curr = &mut curr.as_mut().unwrap().next;
    }
    return head.unwrap().next;
}


#[test]
fn debug() {
    let ll = to_list(vec![0,1,2,3,4,5,6]);
    println!("{:?}", ll);
}

#[test]
fn test() {
    let ll = to_list(vec![1,2,-3,3,1]);
    let ans = to_list(vec![3,1]);
    assert_eq!(ans, remove_zero_sum_sublists(ll));

    let ll = to_list(vec![1,2,3,-3,4]);
    let ans = to_list(vec![1,2,4]);
    assert_eq!(ans, remove_zero_sum_sublists(ll));
    
    let ll = to_list(vec![1,2,3,-3,-2]);
    let ans = to_list(vec![1]);
    assert_eq!(ans, remove_zero_sum_sublists(ll));

    let ll = to_list(vec![1,2,1,-1,1]);
    let ans = to_list(vec![1,2,1]);
    assert_eq!(ans, remove_zero_sum_sublists(ll));
}
