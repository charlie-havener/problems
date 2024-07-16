use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}


pub fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {

    // recursive backtracking with early exit conditions when target is found
    // generates the L/R path to get to the target from the root
    fn recurse(root: Option<Rc<RefCell<TreeNode>>>, target: i32, path: &mut String) -> bool {
        
        if root.is_none() { return false }
        let root = root.as_ref().unwrap();
        let node = root.borrow();

        if node.val == target { return true }

        path.push('L');
        if recurse(node.left.clone(), target, path) { return true }

        path.pop();
        path.push('R');
        if recurse(node.right.clone(), target, path) { return true }

        path.pop();
        return false;
    }

    // build the L/R paths to the targets
    let (mut t_path, mut d_path) = (String::new(), String::new());
    recurse(root.clone(), start_value, &mut t_path);
    recurse(root.clone(), dest_value, &mut d_path);

    let mut t_path = t_path.chars().peekable();
    let mut d_path = d_path.chars().peekable();

    // remove the matching portions, so the 'root' is the LCA
    // start != destination so won't get into an endless loop
    while t_path.peek() == d_path.peek() {
        t_path.next();
        d_path.next();
    }

    // everything from start would be an 'Up' motion and 
    return t_path.map(|_| 'U').collect::<String>() + &d_path.collect::<String>();
}

#[test]
fn tests() {
    let three = Some(Rc::new(RefCell::new(TreeNode{val: 3, left: None, right: None})));
    let six = Some(Rc::new(RefCell::new(TreeNode{val: 6, left: None, right: None})));
    let four = Some(Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: None})));

    let one = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: three, right: None})));
    let two = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: six, right: four})));

    let root = Some(Rc::new(RefCell::new(TreeNode{val: 5, left: one, right: two})));

    let (s,d) = (3,6);
    assert_eq!(String::from("UURL"), get_directions(root,s,d));



    
    let one = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
    let root = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: one, right: None})));
    let (s,d) = (2,1);
    assert_eq!(String::from("L"), get_directions(root,s,d));



    let one = Some(Rc::new(RefCell::new(TreeNode{val: 1, left: None, right: None})));
    let root = Some(Rc::new(RefCell::new(TreeNode{val: 2, left: one, right: None})));
    let (s,d) = (1,2);
    assert_eq!(String::from("U"), get_directions(root,s,d));
}
