use std::rc::Rc;
use std::cell::RefCell;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, left: None, right: None }
    }
}


pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {

    if root.is_none() { return Vec::<i32>::new(); }

    fn helper(root: Rc<RefCell<TreeNode>>, ans: &mut Vec<i32>) {
        let r = root.borrow();
        if r.left.is_some() {
            helper(r.left.clone().unwrap(), ans);
        }
        if r.right.is_some() {
            helper(r.right.clone().unwrap(), ans);
        }
        ans.push(r.val);
    }

    let mut ans: Vec<i32> = Vec::new();
    helper(root.clone().unwrap(), &mut ans);
    return ans;
}


#[test]
fn filled_tree() {
    let a = Some(Rc::new(RefCell::new(TreeNode {val: 6, left: None, right: None })));
    let b = Some(Rc::new(RefCell::new(TreeNode {val: 5, left: None, right: None })));
    let c = Some(Rc::new(RefCell::new(TreeNode {val: 4, left: None, right: None })));

    let d = Some(Rc::new(RefCell::new(TreeNode {val: 3, left: None, right: a })));
    let e = Some(Rc::new(RefCell::new(TreeNode {val: 2, left: c, right: b })));

    let root = Some(Rc::new(RefCell::new(TreeNode {val: 1, left: e, right: d })));
    assert_eq!(vec![4,5,2,6,3,1], postorder_traversal(root));


}

#[test]
fn empty_tree() {
    assert_eq!(Vec::<i32>::new(), postorder_traversal(None));
}
