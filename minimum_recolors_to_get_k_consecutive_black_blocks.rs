pub fn minimum_recolors(blocks: String, k: i32) -> i32 {

    let blocks = blocks.as_bytes();
    let mut ws = 0;

    for i in 0..k as usize {
        if blocks[i] == b'W' {
            ws += 1;
        }
    }

    let mut ans = ws;
    for i in (k as usize)..blocks.len() {
        if blocks[i] == b'W' {
            ws += 1;
        }
        if blocks[i - k as usize] == b'W' {
            ws -= 1;
        }
        ans = ans.min(ws);
    }

    return ans;
}
