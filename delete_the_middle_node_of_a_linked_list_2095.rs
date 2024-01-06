#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val, }
    }

    fn from(vals: Vec<i32>) -> Option<Box<Self>> {
        let mut head = Some(Box::new(ListNode { next:None, val: 0 }));
        let mut node = &mut head;

        for v in vals{
            let new_node = Box::new(ListNode { next: None, val: v });
            node.as_mut().unwrap().next = Some(new_node);
            node = &mut node.as_mut().unwrap().next;
        }
        return head.unwrap().next;
    }
}

pub fn print_list(mut head: &Option<Box<ListNode>>) {
    while let Some(curr) = head.as_ref() {
        print!("{:?} -> ", curr.val);
        head = &head.as_ref().unwrap().next;
    }
    print!("None\n");
}

pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut count: usize = 0;
    let mut node = &head;
    while let Some(curr) = node.as_ref() {
        count += 1;
        node = &curr.next;
    }

    // if there is only a single node, then the returned list will have no nodes.
    if count <= 1 { return None };

    // get node BEFORE the node that needs to be removed
    count = count/2 - 1;
    let mut node = &mut head;
    while let Some(curr) = node.as_mut() {
        if count == 0 {
            let to_delete = curr.next.take();
            curr.next = to_delete.unwrap().next;
            print_list(&head);
            return head;
        }
        count -= 1;
        node = &mut node.as_mut().unwrap().next;
    }

    unreachable!(); // getting here means a node wasn't removed, which cannot happen
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_and_print() {
        let list = ListNode::from(vec![1,2,3,4,5,6]);
        print_list(&list);

        let list = ListNode::from(vec![]);
        print_list(&list);

        let list = ListNode::from(vec![0]);
        print_list(&list);

    }
    
    #[test]
    fn delete() {
        let mut list: Option<Box<ListNode>>;
        let mut ans: Option<Box<ListNode>>;

        list = ListNode::from(vec![1,3,4,7,1,2,6]);
        ans = delete_middle(list);
        assert_eq!(ans, ListNode::from(vec![1,3,4,1,2,6]));
        
        list = ListNode::from(vec![1,2,3,4]);
        ans = delete_middle(list);
        assert_eq!(ans, ListNode::from(vec![1,2,4]));

        list = ListNode::from(vec![2,1]);
        ans = delete_middle(list);
        assert_eq!(ans, ListNode::from(vec![2]));

        list = ListNode::from(vec![1]);
        ans = delete_middle(list);
        assert_eq!(ans, ListNode::from(vec![]));
    }

}
