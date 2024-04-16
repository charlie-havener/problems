#[allow(unused)]
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(unused)]
impl TreeNode {
    fn new(val: i32) -> Self {
        Self {val, left: None, right: None}
    }
}

use std::rc::Rc;
use std::cell::RefCell;

pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32)
    -> Option<Rc<RefCell<TreeNode>>> {

    if depth == 1 {
        let r = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        r.as_ref().unwrap().borrow_mut().left = root;
        return r;
    }

    let mut stack = vec![Rc::clone(root.as_ref().unwrap())];
    let mut next_stack: Vec<Rc<RefCell<TreeNode>>> = vec![];

    let mut level = 1;
    loop {
        level += 1;
        if level == depth { break; }
        while let Some(node) = stack.pop() {
            let n = node.borrow();
            if n.left.is_some() {
                let l = Rc::clone(n.left.as_ref().unwrap());
                next_stack.push(l);
            }
            if n.right.is_some() {
                let r = Rc::clone(n.right.as_ref().unwrap());
                next_stack.push(r);
            }
        }
        std::mem::swap(&mut stack, &mut next_stack);
    }

    println!("stack: {:?}\n", stack);

    while let Some(node) = stack.pop() {
        let mut n = node.borrow_mut();
        let l = n.left.take();
        n.left = Some(Rc::new(RefCell::new(TreeNode {val, left: l, right: None})));
        let r = n.right.take();
        n.right = Some(Rc::new(RefCell::new(TreeNode {val, left: None, right: r})));
    }
    return root;
}


#[test]
fn tests() {
    //let a = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    //let b = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    //let c = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    //let d = Some(Rc::new(RefCell::new(TreeNode {val: 2, left: a, right: b})));
    //let e = Some(Rc::new(RefCell::new(TreeNode {val: 6, left: c, right: None})));

    //let e = Some(Rc::new(RefCell::new(TreeNode {val: 4, left: d, right: e})));

    //println!("{:?}", add_one_row(e, 1, 2));


    let a = Some(Rc::new(RefCell::new(TreeNode::new(4))));

    let b = Some(Rc::new(RefCell::new(TreeNode {val: 2, left: a, right: None})));
    let c = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    let d = Some(Rc::new(RefCell::new(TreeNode {val: 1, left: b, right: c})));

    println!("{:?}", add_one_row(d, 5, 4));
}
