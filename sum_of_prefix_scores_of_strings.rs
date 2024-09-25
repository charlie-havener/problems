#[derive(Debug)]
pub struct TrieNode {
    count: usize,
    children: [Option<Box<TrieNode>>; 26],
}

impl TrieNode {
    const D: Option<Box<TrieNode>> = None;
    fn new() -> Self {
        Self { count: 1, children: [Self::D; 26] }
    }
}


pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
    
    let mut trie = TrieNode::new();
    trie.count = 0;

    // first pass construct the trie
    // keeping track of the number of occurances at each
    for w in &words {
        let mut chars = w.chars();
        let mut r = &mut trie;
        while let Some(letter) = chars.next() {
            let idx = (letter as u8 - b'a') as usize;
            match &mut r.children[idx] {
                Some(ref mut child) => {
                    child.count += 1;
                }
                None => {
                    r.children[idx] = Some(Box::new(TrieNode::new()));
                }
            }
            r = r.children[idx].as_mut().unwrap();
        }
    }

    // second pass count sum the counts at each step
    // and add them to answer
    let mut ans = Vec::with_capacity(words.len());
    for w in words {
        let mut chars = w.chars();
        let mut r = &trie;
        let mut cnt = 0;
        while let Some(letter) = chars.next() {
            let idx = (letter as u8 - b'a') as usize;
            match &r.children[idx] {
                Some(child) => cnt += child.count,
                None => panic!("how tf"),
            }
            r = r.children[idx].as_ref().unwrap();
        }
        ans.push(cnt as i32);
    }

    return ans;
}

#[test]
fn tests() {
    let w = vec![String::from("abc"),String::from("ab"),String::from("bc"),String::from("b")];
    assert_eq!(vec![5,4,3,2], sum_prefix_scores(w));

    let w = vec![String::from("abcd")];
    assert_eq!(vec![4], sum_prefix_scores(w));

    let w = vec![String::from("abcdefghijklmnop"),String::from("abcdefghijklmno")];
    assert_eq!(vec![31,30], sum_prefix_scores(w));
}
