use std::collections::{BTreeMap, HashMap};

struct TrieNode<'a> {
    children: BTreeMap<&'a str, TrieNode<'a>>,
    serialization: String,
}

impl<'a> TrieNode<'a> {
    fn new() -> Self {
        return TrieNode {children: BTreeMap::new(), serialization: String::new()};
    }

    fn add_path(&mut self, path: &'a Vec<String>) {
        let mut root = self;
        for f in path {
            root = root.children.entry(f).or_insert(TrieNode::new());
        }
    }

    fn serialize(&mut self, desc: &mut HashMap<String, usize>) {
        if self.children.len() == 0 {
            return;
        }
        let mut ser = String::new();
        for (&k, v) in self.children.iter_mut() {
            v.serialize(desc);
            ser.push_str(&format!("{}({})", k, v.serialization));
        }
        desc.entry(ser.clone()).and_modify(|c| *c += 1).or_insert(1);
        self.serialization = ser;
    }

    fn has_a_dupe_node(&self, path: &'a Vec<String>, desc: &HashMap<String, usize>) -> bool {
        let mut root = self;
        for f in path {
            root = root.children.get(f.as_str()).unwrap();
            if *desc.get(&root.serialization).unwrap_or(&0) > 1 { return true }
        }
        return false;
    }
}


pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {

    let mut root = TrieNode::new();
    for p in &paths {
        root.add_path(p);
    }

    let mut desc = HashMap::new();
    root.serialize(&mut desc);

    let mut ans = Vec::new();
    for p in &paths {
        if !root.has_a_dupe_node(p, &desc) {
            ans.push(p.clone());
        }
    }

    return ans;
}
