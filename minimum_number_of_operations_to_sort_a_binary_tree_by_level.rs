use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

    let mut ans: i32 = 0;
    let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    q.push_back(root.unwrap());

    while !q.is_empty() {
        
        let mut v = Vec::new();
        let mut v2 = Vec::new();
        for _ in 0..q.len() {
            v2.push(q[0].borrow().val);
            v.push(q.pop_front().unwrap());
        }
        v2.sort();
        let mut diff = 0;
        for (idx,val) in v2.iter().enumerate() {
            if *val != v[idx].borrow().val {
                diff += 1;
            }
        }
        ans += 0.max(diff-1);
        for node in v {
            if node.borrow().left.is_some() {
                q.push_back(node.borrow_mut().left.take().unwrap());
            }
            if node.borrow().right.is_some() {
                q.push_back(node.borrow_mut().right.take().unwrap());
            }
        }
    }


    return ans;
}
