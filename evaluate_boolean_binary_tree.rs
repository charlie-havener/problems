use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}


pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let r = root.unwrap();
    let x = r.borrow();
    match x.val {
        0 => return false,
        1 => return true,
        2 => {
            let left = x.left.clone();
            let right = x.right.clone();
            return evaluate_tree(left) || evaluate_tree(right);
        },
        3 => {
            let left = x.left.clone();
            let right = x.right.clone();
            return evaluate_tree(left) && evaluate_tree(right);
        },
        _ => unreachable!(),
    }
}
