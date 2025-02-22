use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, left: None, right: None }
    }
}

struct FindElements {
    values: HashSet<i32>
}

impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        
        let mut queue: Vec<Rc<RefCell<TreeNode>>> = Vec::with_capacity(21);
        let mut hs = HashSet::new();
        
        if let Some(n) = root {
            n.borrow_mut().val = 0;
            queue.push(Rc::clone(&n));
        }

        while let Some(node) = queue.pop() {
            hs.insert(node.borrow().val);
            if let Some(ref l) = node.borrow().left {
                l.borrow_mut().val = node.borrow().val * 2 + 1;
                queue.push(Rc::clone(l));
            }
            if let Some(ref r) = node.borrow().right {
                r.borrow_mut().val = node.borrow().val * 2 + 2;
                queue.push(Rc::clone(r));
            }
        }

        return Self {values: hs};
    }

    fn find(&self, target: i32) -> bool {
        self.values.contains(&target)
    }
}

#[test]
fn tests() {

    let x = 0_usize;

    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let fe = FindElements::new(root.clone());
    let a = root.is_some();
}
