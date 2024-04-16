struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {val, left: None, right: None}
    }
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
    if root.is_none() {
        return 0;
    } 
    
    let r = root.unwrap();
    let sum = sum*10 + r.borrow().val;


    if r.borrow().left.is_none() && r.borrow().right.is_none() {
        return sum;
    }

    let (mut left, mut right) = (0,0);
    if r.borrow().left.is_some() {
        left = dfs(r.borrow_mut().left.take(), sum);
    }
    if r.borrow().right.is_some() {
        right = dfs(r.borrow_mut().right.take(), sum);
    }
    return left + right;
}

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    return dfs(root, 0);
}

#[test]
fn tests() {
    let a = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let b = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let c = Some(Rc::new(RefCell::new(TreeNode::new(0))));

    let d = Some(Rc::new(RefCell::new(TreeNode {val: 9, left: a, right: b})));

    let root = Some(Rc::new(RefCell::new(TreeNode {val: 4, left: d, right: c})));

    assert_eq!(1026, sum_numbers(root));

}
