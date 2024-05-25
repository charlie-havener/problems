pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {

    fn recurse<'a>(pos: usize, s: &'a str, word_dict: &'a Vec<String>, current: &mut Vec<&'a str>, ans: &mut Vec<String>) {
        if pos >= s.len() {
            ans.push(current.join(" "));
            return;
        }

        for word in word_dict { 
            let l = word.len();
            if pos + l > s.len() { continue }
            if &s[pos..(pos+l)] == word {
                current.push(word);
                recurse(pos + l, s, word_dict, current, ans);
                current.pop();
            }
        }

        return;
    }

    let mut ans = vec![];
    recurse(0, &s, &word_dict,&mut vec![], &mut ans);
    return ans;
}


#[test]
fn tests() {
    let s = String::from("catsanddog");
    let word_dict = vec![String::from("cat"),String::from("cats"),String::from("and"),String::from("sand"),String::from("dog")];
    println!("{:?}", word_break(s, word_dict));

    let s = String::from("pineapplepenapple");
    let word_dict = vec![String::from("apple"),String::from("pen"),String::from("applepen"),String::from("pine"),String::from("pineapple")];
    println!("{:?}", word_break(s, word_dict));

    let s = String::from("catsandog");
    let word_dict = vec![String::from("cats"),String::from("dog"),String::from("sand"),String::from("and"),String::from("cat")];
    println!("{:?}", word_break(s, word_dict));
}

