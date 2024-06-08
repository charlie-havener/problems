use std::collections::HashMap;

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: Option<String>,
}

impl TrieNode {
    fn new() -> Self {
        Self { children: HashMap::new(), is_end: None }
    }

    fn insert(&mut self, word: &str) {
        let mut curr = self;

        for c in word.chars() {
            curr = curr.children.entry(c).or_default();
        }
        
        curr.is_end = Some(word.to_string());
    }

    fn contains_prefix(&self, word: &str) -> Option<String> {
        let mut curr = self;

        for c in word.chars() {
            match curr.children.get(&c) {
                Some(node) => {
                    curr = node;
                    if curr.is_end.is_some() {
                        return curr.is_end.clone();
                    }
                },
                None => return None,
            }
        }
        return None;
    }
}


pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
    let mut trie = TrieNode::new();
    let mut s = sentence.split(" ");
    let mut ans = Vec::new();

    for d in dictionary {
        trie.insert(&d);
    }


    while let Some(w) = s.next() {
        if let Some(r) = trie.contains_prefix(&w) {
            ans.push(r);
        } else {
            ans.push(w.to_string());
        }
    }

    return ans.join(" ");
    

}

#[test]
fn tests() {
    let d = vec![String::from("cat"), String::from("bat"), String::from("rat")];
    let s = String::from("the cattle was rattled by the battery");
    assert_eq!(String::from("the cat was rat by the bat"), replace_words(d, s));

    let d = vec![String::from("a"), String::from("b"), String::from("c")];
    let s = String::from("aadsfasf absbs bbab cadsfafs");
    assert_eq!(String::from("a a b c"), replace_words(d, s));
}
