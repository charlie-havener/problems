use std::collections::HashMap;
use std::collections::HashSet;
use std::cell::RefCell;
use std::rc::Rc;


struct ListNode {
    values: HashSet<String>,
    count: i32,
    next: Option<Rc<RefCell<ListNode>>>,
    prev: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(v: i32) -> Self {
        return Self {
            values: HashSet::new(),
            count: v,
            next: None,
            prev: None,
        };
    }
}

struct AllOne {
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
    keys: HashMap<String, Rc<RefCell<ListNode>>>,
    largest: i32,
}

impl AllOne {

    fn new() -> Self {
        
        // head and tail will be dummy nodes in the linked list
        let mut head =  Some(Rc::new(RefCell::new(ListNode::new(0))));
        let mut tail =  Some(Rc::new(RefCell::new(ListNode::new(-1))));
        head.as_mut().unwrap().borrow_mut().next = tail.clone();
        tail.as_mut().unwrap().borrow_mut().prev = head.clone();

        return Self {
            head,
            tail,
            keys: HashMap::new(),
            largest: 0,
        };
    }

    // all adds will be added just before tail. The count will be one greater than the node
    // currently previous to tail. Dummy head has count = 0 so this still works on an 'empty'
    // list where the first node with count 1 has yet to be added.
    fn add_node(&mut self) {
        let prev_tail = &self.tail.as_ref().unwrap().borrow().prev;
        let count = prev_tail.as_ref().unwrap().borrow().count;
        let mut new_node = Some(Rc::new(RefCell::new(ListNode::new(count))));
        new_node.as_mut().unwrap().borrow_mut().prev = prev_tail.clone();
        new_node.as_mut().unwrap().borrow_mut().next = self.tail.clone();
        prev_tail.as_ref().unwrap().borrow_mut().next = new_node.clone();
        self.tail.as_ref().unwrap().borrow_mut().prev = new_node.clone();
    }

    fn inc(&mut self, key: String) {
        if let None = self.keys.get(&key) {
            // doesn't exist so the count will be one.
            // add the key to the node with count = 1
            // and add that node if it doesn't already exist
            if self.largest == 0 {
                self.add_node();
            }
            self.tail.as_ref().unwrap().borrow().prev.as_ref().unwrap().borrow_mut().values.insert(key.clone());
            self.keys.entry(key).or_insert(self.tail.as_ref().unwrap().borrow().prev.clone().unwrap());
            return;
        }

        // the node exists, if the count is the largest add a new node
        // remove the string from the values of the node and it to the values
        // of the next node. Update the hashmap to point to the next node
        let node = self.keys.get(&key).unwrap().clone();
        if node.borrow().count == self.largest {
            self.add_node();
        }
        node.borrow_mut().values.remove(&key);
        node.borrow().next.as_ref().unwrap().borrow_mut().values.insert(key.clone());
        self.keys.entry(key).and_modify(|v| *v = node.borrow().next.clone().unwrap());
    }
}

impl std::fmt::Debug for AllOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut node = self.head.clone();
        while let Some(n) = node {
            write!(f, "count: {}, strings: {:?}\n", n.borrow().count, n.borrow().values)?;
            node = n.borrow().next.clone(); 
        }
        return Ok(());
    }
}


#[test]
fn tests() {
    let mut o = AllOne::new();
    o.inc(String::from("a"));
}
