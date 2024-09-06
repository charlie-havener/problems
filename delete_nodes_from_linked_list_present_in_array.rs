struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

use std::collections::HashSet;

pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    
    let nums = nums.iter().fold(HashSet::<i32>::new(), |mut acc, n| {
        acc.insert(*n);
        acc
    });

    let mut dummy_head = Some(Box::new(ListNode{val: -1, next: head}));
    let mut curr = dummy_head.as_mut().unwrap();
    while curr.next.is_some() {
        
        if nums.contains(&curr.next.as_ref().unwrap().val) {
            curr.next = curr.next.as_mut().unwrap().next.take();
        } else {
            curr = curr.next.as_mut().unwrap();
        }
    }
    return dummy_head.unwrap().next;
}
