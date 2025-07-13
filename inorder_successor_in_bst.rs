use std::cell::RefCell;
use std::rc::Rc;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn inorder_successor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {

    let mut last_added = None;
    let mut current = root;

    while let Some(curr_rc) = current.clone() {

        let curr_ref = curr_rc.borrow_mut();

        // there is no left node, so process the current node
        // and move to the right
        if curr_ref.left.is_none() {
            if let Some(last_added_rc) = last_added.clone() {
                if Rc::ptr_eq(&last_added_rc, p.as_ref().unwrap()) {
                    return current.clone();
                }
            }
            last_added = current.clone();
            current = curr_ref.right.clone();
        }

        else {

            // goal: find the predecessor of current
            let mut pred = curr_ref.left.clone();

            // find the rightmost child of the left subtree (this will be the final value of pred)
            while let Some(pred_rc) = pred.clone() {
                let mut pred_ref = pred_rc.borrow_mut();
                if let Some(right_rc) = pred_ref.right.clone() {

                    // there is a thread up already so we break it
                    if Rc::ptr_eq(&right_rc, &curr_rc) {
                        pred_ref.right = None;
                        if let Some(last_added_rc) = last_added.clone() {
                            if Rc::ptr_eq(&last_added_rc, p.as_ref().unwrap()) {
                                return current.clone();
                            }
                        }
                        last_added = current.clone();
                        current = curr_ref.right.clone();
                        break;
                    }
                    else {
                        pred = Some(right_rc);
                    }
                }
                else {
                    // create the thred
                    pred_ref.right = Some(curr_rc.clone());
                    current = curr_ref.left.clone();
                    break;
                }
            }

    }
    None
}

#[test]
fn test() {
    inorder_successor(None, None);
}
