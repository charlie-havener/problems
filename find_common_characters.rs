pub fn common_chars(words: Vec<String>) -> Vec<String> {

    let mut ans = vec![0; 26];
    for c in words[0].as_bytes() {
        ans[(c - b'a') as usize] += 1;
    }

    for word in &words[1..] {
        let mut curr = vec![0; 26];
        for c in word.as_bytes() {
            curr[(c - b'a') as usize] += 1;
        }
        for idx in 0..26 {
            ans[idx] = ans[idx].min(curr[idx]);
        }
    }

    let mut res = vec![];
    for idx in 0..26 {
        for _ in 0..ans[idx] {
            res.push(((((idx as u8) + b'a')) as char).to_string());
        }
    }

    return res;

}

#[test]
fn tests() {
    let w = vec![String::from("bella"),String::from("label"),String::from("roller")];
    println!("{:?}", common_chars(w));
}
