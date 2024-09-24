//use std::collections::HashSet;

#[derive(Debug)]
pub struct TrieNode {
    children: [Option<Box<TrieNode>>; 10]
}

impl TrieNode {
    const D: Option<Box<TrieNode>> = None;
    fn new() -> Self {
        Self { children: [Self::D; 10] }
    }
}


pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {

    // build a Trie from arr1
    let mut root = TrieNode::new();
    for num in arr1 {
        let num = num.to_string();
        let mut num = num.chars(); 
        let mut r = &mut root;
        while let Some(d) = num.next() {
            let idx = d.to_digit(10).unwrap() as usize;
            match r.children[idx] {
                Some(_) => (), // already exists
                None => r.children[idx] = Some(Box::new(TrieNode::new())),
            }
            r = r.children[idx].as_mut().unwrap();
        }
    }
    
    // track and return the longest prefix of an element of arr2 in the trie
    let mut ans = 0;
    for num in arr2 {
        let mut curr = 0;
        let num = num.to_string();
        let mut num = num.chars();
        let mut r = &root;
        while let Some(d) = num.next() {
            let idx = d.to_digit(10).unwrap() as usize;
            match r.children[idx] {
                None => break,
                Some(ref c) => r = c,
            }
            curr += 1;
            ans = ans.max(curr);
        }
    }

    return ans;
}

// pub fn longest_common_prefix(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
//     
//     let mut hs: HashSet<i32> = HashSet::new();
// 
//     for pref in arr1 {
//         let s = pref.to_string();
//         let mut s = s.chars();
//         let mut curr: i32 = 0;
//         while let Some(c) = s.next() {
//             curr = curr*10 + c.to_digit(10).unwrap() as i32;
//             hs.insert(curr);
//         }
//     }
// 
//     let mut ans = 0;
//     for pref in arr2 {
//         let s = pref.to_string();
//         let mut s = s.chars().enumerate();
//         let mut curr: i32 = 0;
//         while let Some((idx, c)) = s.next() {
//             curr = curr*10 + c.to_digit(10).unwrap() as i32;
//             if hs.get(&curr).is_some() {
//                 ans = ans.max(idx + 1);
//             }
//         }
//     }
// 
//     return ans as i32;
// }

#[test]
fn tests() {
    let a = vec![1,10,100];
    let b = vec![1000];
    assert_eq!(3, longest_common_prefix(a, b));

    let a = vec![1,2,3];
    let b = vec![4,4,4];
    assert_eq!(0, longest_common_prefix(a, b));
}
