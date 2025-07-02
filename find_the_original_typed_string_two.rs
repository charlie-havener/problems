const MOD: i32 = 1_000_000_007;

pub fn possible_string_count(word: String, k: i32) -> i32 {

    let k = k as usize;

    if word.len() < k { return 0 } // no way to have a len k word
    if word.len() == k { return 1 } // no removals allowed
    let max_removals = word.len() - k;

    let mut prev: Vec<i32> = vec![1; max_removals + 1];
    let mut curr: Vec<i32> = prev.clone();

    let mut rle = 1;
    let word = word.as_bytes();
    for w_idx in 1..word.len() {
        if word[w_idx] == word[w_idx - 1] {
            rle += 1;
            if w_idx == word.len() - 1 {
                calc(&mut prev, &mut curr, rle);
            }
        } else {
            calc(&mut prev, &mut curr, rle);
            rle = 1;
        }
    }
    
    // mod in rust can be negative so will need to use rem_euclid
    // at the end to make sure it's positive;
    return prev.last().unwrap().rem_euclid(MOD);
}


fn calc(prev: &mut Vec<i32>, curr: &mut Vec<i32>, rle: usize) {

    for i in 1..curr.len() {
        curr[i] = curr[i-1];
        curr[i] = (curr[i] + prev[i]).rem_euclid(MOD);
        if rle <= i {
            curr[i] = (curr[i] - prev[i - rle]).rem_euclid(MOD);
        }
    }

    std::mem::swap(prev, curr);
}


#[test]
fn tests() {

    let word = String::from("aabbccdd");
    let k = 7;
    assert_eq!(5, possible_string_count(word, k));

    let word = String::from("aabbccdd");
    let k = 8;
    assert_eq!(1, possible_string_count(word, k));

    let word = String::from("aaabbb");
    let k = 3;
    assert_eq!(8, possible_string_count(word, k));

    let word = String::from("aaaa");
    let k = 10;
    assert_eq!(0, possible_string_count(word, k));
    
    let word = String::from("aaaabbbbccccddddeeeeffffgggghhhhiiiijjjjkkkkllllmmmmnnnnooooppppqqqqrrrrssssttttuuuuvvvvwwwwxxxxyyyyzzzz");
    let k = 26;
    assert_eq!(595845303, possible_string_count(word, k));

    let word = String::from("aaaabbbbccccddddeeeeffffgggghhhhiiiijjjjkkkkllllmmmmnnnnooooppppqqqqrrrrssssttttuuuuvvvvwwwwxxxxyyyyzzzzaaaabbbbccccddddeeeeffffgggghhhhiiiijjjjkkkkllllmmmmnnnnooooppppqqqqrrrrssssttttuuuuvvvvwwwwxxxxyyyyzzzz");
    let k = 26;
    assert_eq!(621940455, possible_string_count(word, k));

    let word = String::from("aaaabbbbccccddddeeeeffffgggghhhhiiiijjjjkkkkllllmmmmnnnnooooppppqqqqrrrrssssttttuuuuvvvvwwwwxxxxyyyyzzzzaaaabbbbccccddddeeeeffffgggghhhhiiiijjjjkkkkllllmmmmnnnnooooppppqqqqrrrrssssttttuuuuvvvvwwwwxxxxyyyyzzzzaaaabbbbccccddddeeeeffffgggghhhhiiiijjjjkkkkllllmmmmnnnnooooppppqqqqrrrrssssttttuuuuvvvvwwwwxxxxyyyyzzzzaaaabbbbccccddddeeeeffffgggghhhhiiiijjjjkkkkllllmmmmnnnnooooppppqqqqrrrrssssttttuuuuvvvvwwwwxxxxyyyyzzzz");
    let k = 26;
    assert_eq!(857937543, possible_string_count(word, k));

}
