use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, left: None, right: None }
    }
}


pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    
    let mut pq = VecDeque::new();
    let mut level = 0;
    pq.push_back(root.as_ref().unwrap().clone());

    while !pq.is_empty() {

        let level_count = level * 2;

        // reverse if the level is odd
        if level % 2 == 1 {
            let (mut l, mut r): (usize, usize) = (0, level_count - 1);
            while l < r {
                let lv = pq[l].borrow().val;
                let rv = pq[r].borrow().val;
                pq[l].borrow_mut().val = rv;
                pq[r].borrow_mut().val = lv;
                l += 1;
                r -= 1;
            }
        }

        // add the next level if it exists
        if pq[0].borrow().left.is_none() { break }
        for _ in 0..level_count {
            let p = pq.pop_front().unwrap();
            pq.push_back(p.borrow().left.clone().unwrap());
            pq.push_back(p.borrow().right.clone().unwrap());
        }

        level += 1;
    }
    
    return root;
}
