use std::rc::Rc;
use std::cell::RefCell;


struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}


pub fn largest_bst_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut ans = 0;
    let _ = search(root.clone(), &mut ans);
    return ans;
}


fn search(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> (bool, i32, i32, i32) {

    if root.is_none() {
        return (true, 0, i32::MAX, i32::MIN);
    }

    let root = root.as_ref().unwrap();

    let (l_bool, l_size, l_min, l_max) = search(root.borrow().left.clone(), ans);
    let (r_bool, r_size, r_min, r_max) = search(root.borrow().right.clone(), ans);

    let c_val = root.borrow().val;

    if l_bool && r_bool && l_max < c_val && r_min > c_val {
        let size = 1 + l_size + r_size;
        *ans = size.max(*ans);
        return (true, size, l_min.min(c_val), r_max.max(c_val));

    }

    return (false, 0, i32::MIN, i32::MAX);

}
