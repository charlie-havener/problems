#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}



pub fn merge_in_between(mut list1: Option<Box<ListNode>>, a: i32, b: i32, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    
    let mut count = 0;
    let mut curr = &mut list1;
    
    // move up to the pre a_th node
    while count < a-1 {
        curr = &mut curr.as_mut().unwrap().next;
        count += 1;
    }
    println!("pre - a_th node: {:?}", curr);
    
    // keep track of the a_th node
    let mut out = curr.as_mut().unwrap().next.take();
    while count < b {
        out = out.unwrap().next;
        count += 1;
    }
    println!("final removed node: {:?}", out);

    // set the a_th node to be list 2
    curr.as_mut().unwrap().next = list2;


    // traverse list 2 to the last node
    while let Some(_) = curr.as_mut().unwrap().next {
        curr = &mut curr.as_mut().unwrap().next;
    }
    println!("end of list 2: {:?}", curr);

    // add to the end of list 2
    curr.as_mut().unwrap().next = out;

    println!("final: {:?}", list1);

    return list1;
}



#[test]
fn test() {
    let n = Some(Box::new(ListNode::new(5)));
    let n = Some(Box::new(ListNode { val: 9, next: n}));
    let n = Some(Box::new(ListNode { val: 6, next: n}));
    let n = Some(Box::new(ListNode { val: 13, next: n}));
    let n = Some(Box::new(ListNode { val: 1, next: n}));
    let n = Some(Box::new(ListNode { val: 10, next: n}));

    
    let m = Some(Box::new(ListNode::new(102)));
    let m = Some(Box::new(ListNode { val: 101, next: m}));
    let m = Some(Box::new(ListNode { val: 100, next: m}));

    println!("testing list: {:?}", n);
    merge_in_between(n, 3, 4, m);
}
