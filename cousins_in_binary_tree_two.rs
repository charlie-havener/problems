use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {

    if root.is_none() { return root }

    // the root wil have no cousins
    let r = root.as_ref().unwrap().clone();
    r.borrow_mut().val = 0;

    let mut pq: Vec<Rc<RefCell<TreeNode>>> = vec![r];
    while !pq.is_empty() {

        // first get the sum of all values in the next level
        let mut level_sum = 0;
        for node in &pq {
            let n = node.borrow();
            if n.left.is_some() {
                level_sum += n.left.as_ref().unwrap().borrow().val;
            }
            if n.right.is_some() {
                level_sum += n.right.as_ref().unwrap().borrow().val;
            }
        }

        let mut npq: Vec<Rc<RefCell<TreeNode>>> = vec![];
        while let Some(node) = pq.pop() {
            let n = node.borrow();

            // get the sum of the children
            let mut child_sum = 0;
            if n.left.is_some() {
                child_sum += n.left.as_ref().unwrap().borrow().val;
            }
            if n.right.is_some() {
                child_sum += n.right.as_ref().unwrap().borrow().val;
            }

            // replace the child values with their cousins sum
            let diff = level_sum - child_sum;
            if n.left.is_some() {
                n.left.as_ref().unwrap().borrow_mut().val = diff;
            }
            if n.right.is_some() {
                n.right.as_ref().unwrap().borrow_mut().val = diff;
            }

            // add the children to the next queue
            if n.left.is_some() {
                npq.push(n.left.as_ref().unwrap().clone());
            }
            if n.right.is_some() {
                npq.push(n.right.as_ref().unwrap().clone());
            }
        }

        std::mem::swap(&mut pq, &mut npq);
    }
    
    return root;
}
