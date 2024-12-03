pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {

    let search_len = search_word.len();

    let words = sentence.split(' ');
    for (idx, word) in words.enumerate() {
        if &word[0..word.len().min(search_len)] == &search_word {
            return idx as i32 + 1;
        }
    }

    return -1;
}

#[test]
fn tests() {
    let sentence = String::from("i love eating burger");
    let search_word = String::from("burg");
    assert_eq!(4, is_prefix_of_word(sentence, search_word));
    
    let sentence = String::from("this problem is an easy problem");
    let search_word = String::from("pro");
    assert_eq!(2, is_prefix_of_word(sentence, search_word));
    
    let sentence = String::from("i am tired");
    let search_word = String::from("you");
    assert_eq!(-1, is_prefix_of_word(sentence, search_word));
}
