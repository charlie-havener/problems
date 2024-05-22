pub fn partition(s: String) -> Vec<Vec<String>> {
    println!("{:?}", s);


    fn recurse(s: usize, idx: usize, ans: &mut Vec<Vec<String>>) {

        let item = ans[s].clone();
        for i in idx..item.len() {
            if item[i].len() == 1 && i + 1 < item.len() && item[i+1] == item[i] {
                let mut s = item[0..i].to_vec();
                let mut e = item[(i+2)..].to_vec();
                s.push(item[i].clone() + &item[i+1].clone());
                s.append(&mut e);
                ans.push(s);
                recurse(ans.len() - 1, i, ans);
            }
            if i != 0 && i + 1 < item.len() && item[i-1] == item[i+1] {
                let mut s = item[0..(i-1)].to_vec();
                let mut e = item[(i+2)..].to_vec();
                s.push(item[i-1].clone() + &item[i].clone() + &item[i+1].clone());
                s.append(&mut e);
                ans.push(s);
                recurse(ans.len() - 1, i - 1, ans);
            }
        }
    }

    let mut ans: Vec<Vec<String>> = vec![s.split("").fold(Vec::new(), |mut acc, s| {
        if s != "" {
            acc.push(s.to_string());
        }
        acc
    })];
    recurse(0, 0, &mut ans);
    return ans;



}

#[test]
fn tests() {
    let s = String::from("aab");
    println!("{:?}", partition(s));

    let s = String::from("a");
    println!("{:?}", partition(s));

    let s = String::from("bbabb");
    println!("{:?}", partition(s));
}
