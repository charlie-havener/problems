pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    
    let mut ans = String::with_capacity(s.len() + spaces.len());

    let mut c = s.chars().enumerate();
    let mut s = spaces.into_iter();
    let mut ss = s.next().unwrap_or(i32::MAX);

    while let Some((idx, l)) = c.next() { 
        if idx == ss as usize {
            ans.push(' ');
            ss = s.next().unwrap_or(i32::MAX);
        }
        ans.push(l);
    }

    return ans;
}

#[test]
fn tests() {
    let s = String::from("LeetcodeHelpsMeLearn");
    let spaces = vec![8,13,15];
    assert_eq!(String::from("Leetcode Helps Me Learn"), add_spaces(s, spaces));

    let s = String::from("icodeinpython");
    let spaces = vec![1,5,7,9];
    assert_eq!(String::from("i code in py thon"), add_spaces(s, spaces));

    let s = String::from("spacing");
    let spaces = vec![0,1,2,3,4,5,6];
    assert_eq!(String::from(" s p a c i n g"), add_spaces(s, spaces));
}
