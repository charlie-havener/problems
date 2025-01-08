pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {

    let mut ans = 0;

    for i in 0..(words.len()-1) {
        let w1 = words[i].as_bytes();
        for j in (i+1)..words.len() {
            let w2 = words[j].as_bytes();            
            if w1.len() > w2.len() { continue }
            if w2.starts_with(&w1) && w2.ends_with(&w1) {
                ans+=1;
            }
        }
    }
    return ans;
}


#[test]
fn tests() {
    let words = vec![String::from("a"),String::from("aba"),String::from("ababa"),String::from("aa")];
    assert_eq!(4, count_prefix_suffix_pairs(words));

    let words = vec![String::from("pa"),String::from("papa"),String::from("ma"),String::from("mama")];
    assert_eq!(2, count_prefix_suffix_pairs(words));

    let words = vec![String::from("abab"),String::from("ab")];
    assert_eq!(0, count_prefix_suffix_pairs(words));

    let words = vec![String::from("ab"),String::from("abab")];
    assert_eq!(1, count_prefix_suffix_pairs(words));
}
