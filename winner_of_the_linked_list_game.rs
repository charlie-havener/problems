struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

fn game_result(head: Option<Box<ListNode>>) -> String {
    let mut count = 0; 

    let mut even = head.as_ref().unwrap();
    let mut odd = even.next.as_ref().unwrap();

    loop {
        if even.val > odd.val { count += 1 }
        else { count -= 1 }

        if odd.next.is_some() {
            even = odd.next.as_ref().unwrap();
            odd = even.next.as_ref().unwrap();
        } else { break; }
    }


    return match count.cmp(&0) {
        std::cmp::Ordering::Equal => String::from("Tie"),
        std::cmp::Ordering::Less => String::from("Odd"),
        std::cmp::Ordering::Greater => String::from("Even"),
    }
}

