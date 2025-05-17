pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {

    let hamming_check = |a: &str, b: &str| -> bool {
        let mut d = 0;
        let a = a.as_bytes();
        let b = b.as_bytes();
        for i in 0..a.len() {
            if a[i] != b[i] {
                d += 1;
            }
            if d > 1 { return false }
        }
        if d == 1 { return true }
        return false;
    };

    let mut store = Vec::with_capacity(words.len());
    store.push((1, -1));

    for i in 1..words.len() {
        let mut v = 1;
        let mut idx = -1;
        for j in 0..i {
            if groups[i] == groups[j] {
                continue;
            }
            if words[i].len() != words[j].len() {
                continue;
            }
            if !hamming_check(&words[i], &words[j]) {
                continue;
            }
            // all conditions are met, update if better.
            if store[j].0 + 1 > v {
                v = store[j].0 + 1;
                idx = j as i32;
            }
        }
        
        store.push((v,idx));
    }
    

    let mut m = 0;
    for (idx, (v, _)) in store.iter().enumerate() {
        if *v > store[m as usize].0 {
            m = idx as i32;
        }
    }
    println!("{m}");

    let mut ans: Vec<String> = Vec::with_capacity(store[m as usize].0 as usize);
    loop {
        ans.push(words[m as usize].clone());
        m = store[m as usize].1;
        if m < 0 { break }
    }
    ans.reverse();
    return ans;
}
