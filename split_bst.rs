use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {
            val, left: None, right: None
        }
    }
}

pub fn split_bst(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    
    // first will values less than or equal to target
    // second will be all that are greater than target
    // 
    // target may not necessarily be in the tree
    
    if root.is_none() {
        return vec![None, None];
    }

    let r = root.as_ref().unwrap();
    let mut n = r.borrow_mut();
    if n.val > target {
        let left = split_bst(n.left.clone(), target);
        n.left = left[1].clone();
        return vec![left[0].clone(), root.clone()];
    }
    else {
        let right = split_bst(n.right.clone(), target);
        n.right = right[0].clone();
        return vec![root.clone(), right[1].clone()];
    }








}
