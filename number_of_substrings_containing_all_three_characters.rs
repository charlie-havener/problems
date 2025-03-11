pub fn number_of_substrings(s: String) -> i32 {

    let s = s.as_bytes();
    let mut store = [0; 3]; // counts of a,b,c
    let mut ans = 0;
    let mut present_count = 0;
    let mut start = 0;

    for (idx, letter) in s.iter().enumerate() {

        let i = (letter - b'a') as usize;

        if store[i] == 0 {
            present_count += 1;
        }
        store[i] += 1;

        while present_count == 3 {
            ans += (s.len() - idx) as i32;
            if store[(s[start] - b'a') as usize] == 1 {
                present_count -= 1;
            }
            store[(s[start] - b'a') as usize] -= 1;
            start += 1;
        }
    }

    return ans;
}
