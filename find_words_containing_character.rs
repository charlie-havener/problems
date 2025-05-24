pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {

    let mut ans = vec![];
    for (i, w) in words.into_iter().enumerate() {
        for c in w.chars() {
            if c == x {
                ans.push(i as i32);
                break;
            }
        }
    }
    return ans;
}
