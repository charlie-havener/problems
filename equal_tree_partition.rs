use std::rc::Rc;
use std::cell::RefCell;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}


pub fn check_equal_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    
    get_sums(Rc::clone(root.as_ref().unwrap()));
    let target = root.as_ref().unwrap().borrow().val;
    return find_answer(Rc::clone(root.as_ref().unwrap()), target);

}


fn find_answer(root: Rc<RefCell<TreeNode>>, target: i32) -> bool {

    // if half the total sum is within a child then return true
    if let Some(left) = &root.borrow().left {
        if left.borrow().val * 2 == target {
            return true;
        }
    }
    if let Some(right) = &root.borrow().right {
        if right.borrow().val * 2 == target {
            return true;
        }
    }


    // if not then we need to check if left or right has the answer.
    // if either of them does, then return true
    let mut ans = false;
    if let Some(left) = &root.borrow().left {
        ans = find_answer(Rc::clone(left), target);
    }

    // no need to travers the right if answer was found in left
    if ans { return ans }
    if let Some(right) = &root.borrow().right {
        ans = find_answer(Rc::clone(right), target);
    }

    return ans;
}


fn get_sums(root: Rc<RefCell<TreeNode>>) {
    
    let mut v = root.borrow().val;

    if let Some(left) = &root.borrow().left {
        get_sums(Rc::clone(left));
        v += left.borrow().val;
    }

    if let Some(right) = &root.borrow().right {
        get_sums(Rc::clone(right));
        v += right.borrow().val;
    }

    root.borrow_mut().val = v;
}
