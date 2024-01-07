#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next:None, val, }
    }

    fn from(vals: Vec<i32>) -> Option<Box<ListNode>>{
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut node = &mut dummy;
        for v in vals {
            let new_node = Box::new(ListNode::new(v));
            node.as_mut().unwrap().next = Some(new_node);
            node = &mut node.as_mut().unwrap().next;
        }
        return dummy.unwrap().next.take();
    }
}

fn print_list(head: &Option<Box<ListNode>>) {
    let mut node = head;
    while let Some(curr) = node.as_ref(){
        print!("{} -> ", curr.val);
        node = &curr.next;
    }
    println!("None");
}

fn reverse(head: &mut Option<Box<ListNode>>) {
    let tail = &mut None;
    while let Some(curr) = head.as_mut() {
        std::mem::swap(&mut curr.next, tail);
        std::mem::swap(head, tail);
    }
    std::mem::swap(head, tail);
}

pub fn double_it(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // reverse the list
    reverse(&mut head);

    // go through and double each, track the add_one flag
    let mut node = &mut head;
    let mut add_one = false;
    while let Some(curr_inner) = node.as_mut() {
        let mut d = curr_inner.val * 2;
        if add_one { 
            add_one = false;
            d += 1;
        }
        if d >= 10 {
            add_one = true;
            d %= 10;
        }
        curr_inner.val = d;
        node = &mut node.as_mut().unwrap().next;
    }

    // un-reverse the list
    reverse(&mut head);
    
    // add a node with value 1 as new head if flag is still set, then return head
    if add_one {
        let mut new_node = Box::new(ListNode::new(1));
        new_node.next = head.take();
        head = Some(new_node);
    }
    return head;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_and_print() {
        let mut list: Option<Box<ListNode>>;

        list = ListNode::from(vec![1,2,3,4,5]);
        print_list(&list);

        list = ListNode::from(vec![]);
        print_list(&list);

        list = ListNode::from(vec![7,4,2]);
        print_list(&list);
    }

    #[test]
    fn reverse_test() {
        let mut list: Option<Box<ListNode>>;

        list = ListNode::from(vec![1,2,3,4,5]);
        reverse(&mut list);
        assert_eq!(list, ListNode::from(vec![5,4,3,2,1]));

        list = ListNode::from(vec![]);
        reverse(&mut list);
        assert_eq!(list, ListNode::from(vec![]));

        list = ListNode::from(vec![2]);
        reverse(&mut list);
        assert_eq!(list, ListNode::from(vec![2]));
    }

    #[test]
    fn doubling() {
        let mut list: Option<Box<ListNode>>;

        list = ListNode::from(vec![7,4,2]);
        assert_eq!(double_it(list), ListNode::from(vec![1,4,8,4]));

        list = ListNode::from(vec![2,8,6]);
        assert_eq!(double_it(list), ListNode::from(vec![5,7,2]));
    }
}
