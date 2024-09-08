#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

pub fn split_list_to_parts(mut head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    
    // get the length of the list;
    let mut length = 0;
    let mut curr = &head;
    while let Some(node) = curr.as_ref() {
        length += 1;
        curr = &node.next;
    }

    let mut ans = Vec::with_capacity(k as usize);
    let mut k = k;

    while k > 0 {
        if length <= 0 { ans.push(None); k -= 1; continue; }

        let mut l = length / k;
        if l * k < length { l += 1 }

        if head.is_none() {
            ans.push(None);
            continue;
        }

        let mut curr = &mut head;

        for _ in 1..l {
            if curr.as_mut().unwrap().next.is_none() {
                break;
            }
            curr = &mut curr.as_mut().unwrap().next;
        }

        let t = curr.as_mut().unwrap().next.take();
        ans.push(head);
        head = t;

        length -= l;
        k -= 1;

    }

    return ans;
}

#[test]
fn tests() {
    println!("{:?}", split_list_to_parts(None, 10));

    let h = Some(Box::new(ListNode { val: 1, next: None }));
    println!("{:?}", split_list_to_parts(h, 10));

    let h = Some(Box::new(ListNode { val: 3, next: None }));
    let h = Some(Box::new(ListNode { val: 2, next: h }));
    let h = Some(Box::new(ListNode { val: 1, next: h }));
    println!("{:?}", split_list_to_parts(h, 5));

    let h = Some(Box::new(ListNode { val: 10, next: None }));
    let h = Some(Box::new(ListNode { val: 9, next: h }));
    let h = Some(Box::new(ListNode { val: 8, next: h }));
    let h = Some(Box::new(ListNode { val: 7, next: h }));
    let h = Some(Box::new(ListNode { val: 6, next: h }));
    let h = Some(Box::new(ListNode { val: 5, next: h }));
    let h = Some(Box::new(ListNode { val: 4, next: h }));
    let h = Some(Box::new(ListNode { val: 3, next: h }));
    let h = Some(Box::new(ListNode { val: 2, next: h }));
    let h = Some(Box::new(ListNode { val: 1, next: h }));
    println!("{:?}", split_list_to_parts(h, 3));

    let h = Some(Box::new(ListNode { val: 11, next: None }));
    let h = Some(Box::new(ListNode { val: 10, next: h }));
    let h = Some(Box::new(ListNode { val: 9, next: h }));
    let h = Some(Box::new(ListNode { val: 8, next: h }));
    let h = Some(Box::new(ListNode { val: 7, next: h }));
    let h = Some(Box::new(ListNode { val: 6, next: h }));
    let h = Some(Box::new(ListNode { val: 5, next: h }));
    let h = Some(Box::new(ListNode { val: 4, next: h }));
    let h = Some(Box::new(ListNode { val: 3, next: h }));
    let h = Some(Box::new(ListNode { val: 2, next: h }));
    let h = Some(Box::new(ListNode { val: 1, next: h }));
    println!("{:?}", split_list_to_parts(h, 3));
}
