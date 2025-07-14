pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {

    g.sort_unstable();
    s.sort_unstable();

    let mut ans = 0;
    let mut g_ptr = 0;
    let mut s_ptr = 0;

    while s_ptr < s.len() {
        if s[s_ptr] >= g[g_ptr] {
            ans += 1;
            g_ptr += 1;
        }
        if g_ptr >= g.len() {
            break;
        }
        s_ptr += 1;
    }

    return ans;

}
