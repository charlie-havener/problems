pub fn clear_digits(s: String) -> String {
    
    let mut ans = String::with_capacity(s.len() / 2);
    for c in s.chars() {
        if c.is_digit(10) {
            ans.pop();
        } else {
            ans.push(c);
        }
    }
    return ans;
}
