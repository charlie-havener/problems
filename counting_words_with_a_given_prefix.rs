pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    let mut ans = 0;
    for w in words {
        if w.starts_with(&pref) { ans += 1 }
    }
    return ans;
}
