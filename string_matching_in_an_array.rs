pub fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut ans = vec![]; 
    for (idx, word) in words.iter().enumerate() {
        println!("lf: {word:?} {idx}");
        for (idx2, word2) in words.iter().enumerate() {
            println!("checking: {word2:?} {idx2}");
            if idx == idx2 { continue }
            if word2.contains(word)  {
                ans.push(word.clone());
                break;
            }
        }
    }
    return ans;
}
