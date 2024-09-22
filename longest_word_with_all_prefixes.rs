use std::collections::HashSet;

pub fn longest_word(mut words: Vec<String>) -> String {
    words.sort_unstable();
    let mut prefixes: HashSet<&str> = HashSet::new();
    let mut ans = "";

    // add all the word prefixes to the hashset
    // if all the prefixes are in the hashset
    // then the word is a potential answer
    for word in &words {
        let mut is_valid = true;
        for i in 0..word.len()-1 {
            if !prefixes.contains(&word[0..=i]) {
                is_valid = false;
            }
        }
        prefixes.insert(word);

        if is_valid && word.len() > ans.len() {
            ans = word;
        }
    }
    return ans.to_string();
}

#[test]
fn tests() {
    let words = vec![String::from("k"),String::from("ki"),String::from("kir"),String::from("kira"),String::from("kiran")];
    assert_eq!("kiran", longest_word(words));

    let words = vec![String::from("a"),String::from("banana"),String::from("app"),String::from("appl"),String::from("ap"),String::from("apply"),String::from("apple")];
    assert_eq!("apple", longest_word(words));

    let words = vec![String::from("abc"),String::from("bc"),String::from("ab"),String::from("qwe")];
    assert_eq!("", longest_word(words));

}
