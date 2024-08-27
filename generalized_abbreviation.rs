pub fn generate_abbreviations(word: String) -> Vec<String> {
    
    fn helper(idx: usize, allow_nums: bool, word: &str, curr: String, ans: &mut Vec<String>) {

        if idx >= word.len() {
            ans.push(curr.to_string());
            return
        }

        // just adding the char as it is
        let l = word[idx..=idx].chars().next().unwrap();
        let mut n_curr = curr.clone();
        n_curr.push(l);
        helper(idx+1, true, word, n_curr, ans);


        // doing some numbers
        if allow_nums {

            for i in 1..=(word.len() - idx) {
                let l = i.to_string();
                let mut n_curr = curr.clone();
                n_curr.extend(l.chars());
                helper(idx+i, false, word, n_curr, ans);
            }
        }
    }

    let mut ans = Vec::new();
    helper(0, true, &word, String::new(), &mut ans);
    return ans;

}

#[test]
fn tests() {
    let w = String::from("word");
    println!("{:?}", generate_abbreviations(w));

    let w = String::from("a");
    println!("{:?}", generate_abbreviations(w));
}
