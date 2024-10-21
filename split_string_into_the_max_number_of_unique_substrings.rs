use std::collections::HashSet;

pub fn max_unique_split(s: String) -> i32 {

    fn recurse<'a, 'b>(ptr: usize, s: &'a String, hs: &'b mut HashSet<&'a str>) -> i32 {
        let mut ans = 0;
        if ptr >= s.len() { return hs.len() as i32 }

        for e in ptr..s.len() {
            let t = &s[ptr..=e];
            let ins = hs.insert(t);
            if ins {
                ans = ans.max(recurse(e+1, s, hs));
                hs.remove(t);
            }
        }
        return ans;
    }
    let mut hs: HashSet<&str> = HashSet::new();
    return recurse(0, &s, &mut hs);
}


#[test]
fn test() {
    //let s = String::from("ababccc");
    //assert_eq!(5, max_unique_split(s));

    //let s = String::from("aba");
    //assert_eq!(2, max_unique_split(s));

    //let s = String::from("aa");
    //assert_eq!(1, max_unique_split(s));

    let s = String::from("wwwzfvedwfvhsww");
    assert_eq!(11, max_unique_split(s));
}
