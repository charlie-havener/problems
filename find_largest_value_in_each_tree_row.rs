use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn largest_value(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {

    if root.is_none() { return vec![] }

    let mut ans = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());
    
    while !queue.is_empty() {
        let mut large = i32::MIN;
        let l = queue.len();
        for _ in 0..l {
            let t = queue.pop_front().unwrap();
            large = large.max(t.borrow().val);
            if let Some(n) = t.borrow_mut().left.take() {
                queue.push_back(n);
            };
            if let Some(n) = t.borrow_mut().right.take() {
                queue.push_back(n);
            };
        }
        ans.push(large)
    }
    return ans;
}
