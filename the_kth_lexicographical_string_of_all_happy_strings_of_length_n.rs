pub fn get_happy_string(n: i32, mut k: i32) -> String {
    let mut hs_count = 3 * 2_i32.pow(n as u32 - 1);


    if k > hs_count {
        return "".into();
    }

    k -= 1;

    let mut ans = String::with_capacity(n as usize);

    // first char has 3 options
    ans.push( (b'a' + (k/(hs_count/3)) as u8) as char);
    k = k % (hs_count / 3);
    hs_count /= 3;

    // all the rest have 2 options (can't match prev)
    while hs_count > 1 {
        
        // in 'upper' half
        if k >= hs_count / 2 {
            let l = match ans.chars().next_back().unwrap() {
                'c' => 'b',
                _ => 'c',
            };
            ans.push(l);
        }
        // else in 'lower' half
        else {
            let l = match ans.chars().next_back().unwrap() {
                'a' => 'b',
                _ => 'a',
            };
            ans.push(l);
        }

        k = k % (hs_count / 2);
        hs_count /= 2;
    }

    return ans
}
