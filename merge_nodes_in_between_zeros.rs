#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

fn create_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
    if vals.is_empty() { return None }

    let mut dh = ListNode::new(-1);
    for v in vals.into_iter().rev() {
        let mut node = Some(Box::new(ListNode::new(v)));
        let tmp = dh.next.take();
        node.as_mut().unwrap().next = tmp;
        dh.next = node;
    }
    return dh.next;
}


pub fn merge_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    
    let mut h = head.as_mut().unwrap();
    while let Some(n) = h.next.as_mut() {
        match n.val {
            0 => {
                match n.next {
                    None => h.next = None,
                    Some(_) => h = h.next.as_mut().unwrap(),
                }
            },
            v => {
                h.val += v;
                h.next = n.next.take();
            }
        }
    }
    return head;
}

#[test]
fn tests() {
    let v = vec![0,3,1,0,4,5,2,0];
    let head = create_list(v);
    assert_eq!(create_list(vec![4,11]), merge_nodes(head));

    let v = vec![0,1,0,3,0,2,2,0];
    let head = create_list(v);
    assert_eq!(create_list(vec![1,3,4]), merge_nodes(head));
}



