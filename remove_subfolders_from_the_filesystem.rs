use std::collections::HashMap;


struct TrieNode<'a> {
    children: HashMap<&'a str, TrieNode<'a>>,
    is_end: bool,
}

impl<'a> TrieNode<'a> {
    fn new() -> Self {
        return 
            Self {
                children: HashMap::new(),
                is_end: false,
            };
    }

    fn add_path(&mut self, path: &'a str) -> () {
        let mut root = self;
        let mut items = path.split('/');
        while let Some(it) = items.next() {
            if it.len() == 0 { continue }
            root = root.children.entry(it).or_insert(TrieNode::new());
        }
        root.is_end = true;
    }

    fn is_subfolder(&self, path: &'a str) -> bool {
        let mut root = self;
        let mut items = path.split('/').peekable();
        while let Some(it) = items.next() {
            if it.len() == 0 { continue }
            root = root.children.get(it).unwrap();
            if root.is_end && items.peek().is_some() {
                return true;
            }
        }
        return false;
    }
}


pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {

    let mut ans = Vec::new();
    let mut root = TrieNode::new();

    for f in &folder {
        root.add_path(f);
    }

    for f in &folder {
        if !root.is_subfolder(f) {
            ans.push(f.clone());
        }
    }
    
    println!("{ans:?}");
    return ans;
}

#[test]
fn tests() {

    let folder = vec![String::from("/a"),String::from("/a/b"),String::from("/c/d"),String::from("/c/d/e"),String::from("/c/f")];
    remove_subfolders(folder);

    let folder = vec![String::from("/a"),String::from("/a/b/c"),String::from("/a/b/d")];
    remove_subfolders(folder);

    let folder = vec![String::from("/a/b/c"),String::from("/a/b/ca"),String::from("/a/b/d")];
    remove_subfolders(folder);
}
