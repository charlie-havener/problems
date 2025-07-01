pub fn possible_string_count(word: String) -> i32 {
    let mut ans = 1;
    // string only has [a-z], so set prev to be an impossible char
    let mut prev = '_';
    for c in word.chars() {
        if c == prev { ans += 1; }
        else { prev = c; }
    }
    return ans;
}
