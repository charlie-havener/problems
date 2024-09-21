use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TreeNode {
    header: String,
    children: Vec<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(s: String) -> Self {
        Self { header: s, children: Vec::new() }
    }
}

pub fn find_smallest_region(regions: Vec<Vec<String>>, region1: String, region2: String) -> String {

    // hashmap values are a tuple of the node, and a flag for if the node is a root
    let mut nodemap: HashMap<&String, (Rc<RefCell<TreeNode>>, bool)> = HashMap::new();
    
    for r in &regions {
        let p = &r[0];

        // insert the node into the hashmap if it doesn't already exist
        // since the first element is the parent, don't update the count
        nodemap
            .entry(p)
            .or_insert((Rc::new(RefCell::new(TreeNode::new(p.to_string()))), true)); 

        // insert the child with a count of 1 if it doesn't already exist
        // or increment the count by 1 if it does
        // add the child to the children of the parent
        for child in &r[1..] {
            nodemap
                .entry(child)
                .and_modify(|(_, c)| *c = false)
                .or_insert((Rc::new(RefCell::new(TreeNode::new(child.to_string()))), false));
             
            // we know that parent and child were inserted above, so unwrap is safe
            let parent = nodemap.get(p).unwrap();
            let child = nodemap.get(child).unwrap();
            let mut n = parent.0.borrow_mut();
            let c = &child.0;
            n.children.push(Rc::clone(c));
        }
    }

    // collect all the roots under a dummy header.
    // there are potentially multiple roots, with dummy only one LCA traversal is needed
    // because the answer is guaranted to exist
    let root = Rc::new(RefCell::new(TreeNode::new(String::from("Dummy"))));
    let mut r = root.borrow_mut();
    for (_, value) in nodemap.iter() {
        if value.1 {
            r.children.push(Rc::clone(&value.0));
        }
    }
    drop(r);

    return find_lca(Rc::clone(&root), &region1, &region2).1.unwrap();
}

fn find_lca(root: Rc<RefCell<TreeNode>>, region1: &str, region2: &str) -> (usize, Option<String>) {
    
    let n = root.borrow();
    let v = &n.header;
    
    let mut count = 0;
    let mut st = None;

    if v == region1 || v == region2 {
        count += 1;
    }

    for child in &n.children {
        if count >= 2 { break; }
        let (c,s) = find_lca(Rc::clone(child), region1, region2);
        count += c;
        st = s;
    }

    if count < 2 {
        //println!("{:?} -> ({}, None)",v, count);
        return (count, None);
    }
    else if count == 2 {
        //println!("{:?} -> (10, {:?})",v, v.to_string());
        return (10, Some(v.to_string()));
    }
    else {
        //println!("{:?} -> (10, {:?})",v, st);
        return (10, st);
    }
}


#[test]
fn tests() {
    let regions = vec![vec![String::from("Earth"),String::from("North America"),String::from("South America")],vec![String::from("North America"),String::from("United States"),String::from("Canada")],vec![String::from("United States"),String::from("New York"),String::from("Boston")],vec![String::from("Canada"),String::from("Ontario"),String::from("Quebec")],vec![String::from("South America"),String::from("Brazil")]];
    let a = String::from("Quebec");
    let b = String::from("New York");
    assert_eq!(String::from("North America"), find_smallest_region(regions, a, b));

    let regions = vec![vec![String::from("Earth"), String::from("North America"), String::from("South America")],vec![String::from("North America"), String::from("United States"), String::from("Canada")],vec![String::from("United States"), String::from("New York"), String::from("Boston")],vec![String::from("Canada"), String::from("Ontario"), String::from("Quebec")],vec![String::from("South America"), String::from("Brazil")]];
    let a = String::from("Canada");
    let b = String::from("South America");
    assert_eq!(String::from("Earth"), find_smallest_region(regions, a, b));

    let regions = vec![vec![String::from("North America"), String::from("United States"), String::from("Canada")],vec![String::from("United States"), String::from("New York"), String::from("Boston")],vec![String::from("Canada"), String::from("Ontario"), String::from("Quebec")],vec![String::from("South America"), String::from("Brazil")]];
    let a = String::from("Quebec");
    let b = String::from("New York");
    assert_eq!(String::from("North America"), find_smallest_region(regions, a, b));
}
