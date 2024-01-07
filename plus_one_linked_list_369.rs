#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from(vals: Vec<i32>) -> Option<Box<Self>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut node = &mut head;
        for v in vals {
            let new_node = Box::new(ListNode::new(v));
            node.as_mut().unwrap().next = Some(new_node);
            node = &mut node.as_mut().unwrap().next;
        }
        return head.unwrap().next;
    }
}

pub fn print_list(head: &Option<Box<ListNode>>) {
    let mut node = head;
    while let Some(curr) = node.as_ref() {
        print!("{} -> ", curr.val);
        node = &curr.next;
    }
    print!("None\n");
}

fn reverse_list (mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
    let mut prev = None;
    let mut curr = head.take();

    while let Some(mut curr_inner) = curr.take() {
        let next = curr_inner.next.take();
        curr_inner.next= prev.take();
        prev = Some(curr_inner);
        curr = next;
    }

    return prev.take();
}

pub fn plus_one(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    print!("starting: "); print_list(&head);
    // reverse list
    head = reverse_list(head);
    print!("reversed: "); print_list(&head);
    

    // iterate through adding one if flag is set
    let mut add_one: bool = true;
    let mut node = &mut head;
    while let Some(curr) = node.as_mut() {
        if add_one {
            curr.val += 1;
            add_one = false;
        }
        if curr.val == 10 {
            curr.val = 0;
            add_one = true;
        }
        node = &mut curr.next;
    }
    print!("adding done: "); print_list(&head);

    // reverse list
    head = reverse_list(head);
    print!("re-reversed: "); print_list(&head);
    
    // at the end if flag is set, append a node w/ value 1
    if add_one {
        let mut new_node = Box::new(ListNode{next: None, val: 1});
        new_node.next = head.take();
        head = Some(new_node);
        print!("optional added: "); print_list(&head);
    }

    // return it
    return head;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_and_from() {
        let mut list: Option<Box<ListNode>>;

        list = ListNode::from(vec![1,2,3,4,5]);
        print_list(&list);

        list = ListNode::from(vec![]);
        print_list(&list);
    }

    #[test]
    fn reverse() {
        let mut list: Option<Box<ListNode>>;

        list = ListNode::from(vec![1,2,3,4]);
        assert_eq!(reverse_list(list), ListNode::from(vec![4,3,2,1]));
    }

    #[test]
    fn test() {
        let mut list: Option<Box<ListNode>>;
        let mut ans: Option<Box<ListNode>>;
        
        list = ListNode::from(vec![1,2,3]);
        ans = plus_one(list);
        assert_eq!(ans, ListNode::from(vec![1,2,4]));

        list = ListNode::from(vec![0]);
        ans = plus_one(list);
        assert_eq!(ans, ListNode::from(vec![1]));

        list = ListNode::from(vec![9,9]);
        ans = plus_one(list);
        assert_eq!(ans, ListNode::from(vec![1,0,0]));
    }
}
