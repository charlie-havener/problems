use std::rc::Rc;
use std::cell::RefCell;

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}


pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {

    fn check_paths(orig_head: &Option<Box<ListNode>>, curr_head: &Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
            
        if curr_head.is_none() { return true }
        if root.is_none() { return false }

        let h = curr_head.as_ref().unwrap();
        let n = root.as_ref().unwrap();
        let n = n.borrow();

        // if the linked list val is the same as the tree val
        // then have the option of continuing through the tree
        // after moving the linked list head on.
        // if not the same, then we can traverse
        if h.val == n.val {
            if check_paths(orig_head, &h.next, n.left.clone()) { return true }
            if check_paths(orig_head, &h.next, n.right.clone()) { return true }
        }

        if std::ptr::eq(orig_head, curr_head) {
            if check_paths(orig_head, orig_head, n.left.clone()) { return true }
            if check_paths(orig_head, orig_head, n.right.clone()) { return true }
        }

        return false;
    }

    return check_paths(&head, &head, root.clone());
}
