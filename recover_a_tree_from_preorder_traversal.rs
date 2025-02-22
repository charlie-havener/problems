use std::rc::Rc;
use std::cell::RefCell;
use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
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

fn recurse(parent: Rc<RefCell<TreeNode>>, depth: i32, trav: &mut Peekable<Chars>) -> Option<(i32, i32)> {

    if trav.peek().is_none() {
        return None;
    }

    let mut dashes = 0;
    while let Some('-') = trav.peek() {
        dashes += 1;
        trav.next();
    }

    let mut number = 0;
    while let Some(d) = trav.peek() {
        if let Some(d) = d.to_digit(10) {
            number = number * 10 + d as i32;
            trav.next();
        } else {
            break;
        }
    }

    loop {
        if dashes <= depth {
            return Some((dashes, number));
        }

        if parent.borrow().left.is_none() {
            parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(number))));
            if let Some(r) = recurse(parent.borrow_mut().left.clone().unwrap(), dashes, trav) {
                (dashes, number) = r;
                continue;
            }
            else { return None }
        }
        else {
            parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(number))));
            if let Some(r) = recurse(parent.borrow_mut().right.clone().unwrap(), dashes, trav) {
                (dashes, number) = r;
                continue;
            }
            else { return None }
        }
    }

}

pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
    let dummy = Rc::new(RefCell::new(TreeNode::new(-1)));
    let _ = recurse(Rc::clone(&dummy), -1, &mut traversal.chars().peekable());
    return dummy.borrow_mut().left.take();
}


#[test]
fn testA() {
    let a = Some(Rc::new(RefCell::new(TreeNode { val: 3, left: None, right: None } )));
    let b = Some(Rc::new(RefCell::new(TreeNode { val: 4, left: None, right: None } )));
    let c = Some(Rc::new(RefCell::new(TreeNode { val: 6, left: None, right: None } )));
    let d = Some(Rc::new(RefCell::new(TreeNode { val: 7, left: None, right: None } )));

    let e = Some(Rc::new(RefCell::new(TreeNode { val: 2, left: a, right: b } )));
    let f = Some(Rc::new(RefCell::new(TreeNode { val: 5, left: c, right: d } )));

    let g = Some(Rc::new(RefCell::new(TreeNode { val: 1, left: e, right: f } )));
    
    let ans = recover_from_preorder(String::from("1-2--3--4-5--6--7"));
    assert_eq!(g, ans);
}

#[test]
fn testB() {
    let a = Some(Rc::new(RefCell::new(TreeNode { val: 4, left: None, right: None } )));
    let b = Some(Rc::new(RefCell::new(TreeNode { val: 3, left: a, right: None } )));
    let c = Some(Rc::new(RefCell::new(TreeNode { val: 2, left: b, right: None } )));

    let d = Some(Rc::new(RefCell::new(TreeNode { val: 7, left: None, right: None } )));
    let e = Some(Rc::new(RefCell::new(TreeNode { val: 6, left: d, right: None } )));
    let f = Some(Rc::new(RefCell::new(TreeNode { val: 5, left: e, right: None } )));

    let g = Some(Rc::new(RefCell::new(TreeNode { val: 1, left: c, right: f } )));


    let ans = recover_from_preorder(String::from("1-2--3---4-5--6---7"));
    assert_eq!(g, ans);
}

#[test]
fn testC() {
    let a = Some(Rc::new(RefCell::new(TreeNode { val: 90, left: None, right: None } )));
    let b = Some(Rc::new(RefCell::new(TreeNode { val: 349, left: a, right: None } )));
    let c = Some(Rc::new(RefCell::new(TreeNode { val: 88, left: None, right: None } )));
    let d = Some(Rc::new(RefCell::new(TreeNode { val: 401, left: b, right: c } )));
    let e = Some(Rc::new(RefCell::new(TreeNode { val: 1, left: d, right: None } )));
    let ans = recover_from_preorder(String::from("1-401--349---90--88"));
    assert_eq!(e, ans);
}
