use std::rc::Rc;
use std::cell::RefCell;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn get_lonely_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = vec![];
    let mut stack = vec![root];
    let mut new_stack = vec![];

    loop {
        while let Some(node) = stack.pop().flatten() {

            let n = node.borrow();
            match (&n.left, &n.right) {
                (Some(l), Some(r)) => {
                    new_stack.push(Some(Rc::clone(l)));
                    new_stack.push(Some(Rc::clone(r)));
                },
                (Some(c), None) | (None, Some(c)) => {
                    ans.push(c.borrow().val);
                    new_stack.push(Some(Rc::clone(c)));
                },
                (None, None) => (),
            }
        }
        if new_stack.is_empty() { break; }
        std::mem::swap(&mut stack, &mut new_stack);
    }
    return ans;
}
