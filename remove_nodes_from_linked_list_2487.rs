use std::collections::VecDeque;


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val, }
    }

    fn from(vals: Vec<i32>) -> Option<Box<Self>> {
        let mut head = Some(Box::new(Self::new(0)));
        let mut cur = &mut head;
        
        for v in vals {
            let new_node = Box::new(ListNode { next: None, val: v });
            cur.as_mut().unwrap().next = Some(new_node);
            cur = &mut cur.as_mut().unwrap().next;
        }

        return head.unwrap().next.take();
    }
}

pub fn as_vec(head: &Option<Box<ListNode>>) -> Vec<i32>{
    let mut ret: Vec<i32> = vec![];
    let mut cur = head;
    while cur.is_some() {
        ret.push(cur.as_ref().unwrap().val);
        cur = &cur.as_ref().unwrap().next; }
    return ret;
}



pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut stack: VecDeque<Box<ListNode>> = VecDeque::new();
    let mut cur = head;

    // create a stack that contains only the nodes meeting the condition (in order)
    while cur.is_some() {
        let t = cur.as_mut().unwrap().next.take();

        // check if the next node is less than what's at the top of the stack
        // and pop off the stack until the top of the stack is greater
        while cur.is_some() && stack.back().is_some_and(|node| node.val < cur.as_ref().unwrap().val) {
            let tmp = stack.pop_back();
            println!("popped {:?} from stack", tmp.unwrap().val);
        }

        println!("pushed {} to stack", cur.as_ref().unwrap().val);
        stack.push_back(cur.unwrap());
        cur = t;
    }

    println!("stack := {:?}", stack);

    // re-link the nodes in the stack.
    let mut head = Some(Box::new(ListNode { next: None, val: 0 }));
    let mut cur = &mut head;
    loop {
        if stack.len() == 0 { break }
        println!("linking {:?}", stack.front());
        let next_node = stack.pop_front();
        cur.as_mut().unwrap().next = next_node;
        cur = &mut cur.as_mut().unwrap().next;
    }
    

    return head.unwrap().next
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print() {
        let mut list = ListNode::from(vec![5,6,7,8]);
        println!("{:?}", as_vec(&list));
        list = ListNode::from(vec![0,1,2,3,4,5]);
        println!("{:?}", as_vec(&list));
        list = ListNode::from(vec![5,2,13,3,8]);
        println!("{:?}", as_vec(&list));
    }

    #[test]
    fn equality() {
        let list1 = ListNode::from(vec![1,2,3]);
        let list2 = ListNode::from(vec![1,2,3]);
        assert_eq!(list1, list2);

        let list1 = ListNode::from(vec![]);
        let list2 = ListNode::from(vec![]);
        assert_eq!(list1, list2);

    }

    #[test]
    fn remove() {
        let mut ans: Option<Box<ListNode>>;

        ans = remove_nodes(ListNode::from(vec![5,2,13,3,8]));
        assert_eq!(ans, ListNode::from(vec![13, 8]));

        ans = remove_nodes(ListNode::from(vec![1,1,1,1]));
        assert_eq!(ans, ListNode::from(vec![1,1,1,1]));

        ans = remove_nodes(ListNode::from(vec![1,2,3,4,5,6]));
        assert_eq!(ans, ListNode::from(vec![6]));

        ans = remove_nodes(ListNode::from(vec![6,5,4,3,2,1]));
        assert_eq!(ans, ListNode::from(vec![6,5,4,3,2,1]));

        ans = remove_nodes(ListNode::from(vec![2]));
        assert_eq!(ans, ListNode::from(vec![2]));
    }

}
