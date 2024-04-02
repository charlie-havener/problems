pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut firsts = vec![-1;26];
    let s = s.as_bytes().iter().enumerate().fold(vec![0;s.len()], |mut acc, (i,b)| {
        match firsts[(b - b'a') as usize] {
            -1 => {
                firsts[(b - b'a') as usize] = i as i32;
                acc[i] = i;
            },
            _ => {
                acc[i] = firsts[(b - b'a') as usize] as usize;
            }
        }
        return acc;
    });

    let mut firsts = vec![-1;26];
    let t = t.as_bytes().iter().enumerate().fold(vec![0;s.len()], |mut acc, (i,b)| {
        match firsts[(b - b'a') as usize] {
            -1 => {
                firsts[(b - b'a') as usize] = i as i32;
                acc[i] = i;
            },
            _ => {
                acc[i] = firsts[(b - b'a') as usize] as usize;
            }
        }
        return acc;
    });

    return s == t;
}

#[test]
fn tests() {
    let s = String::from("egg");
    let t = String::from("add");
    assert_eq!(true, is_isomorphic(s,t));

    let s = String::from("foo");
    let t = String::from("bar");
    assert_eq!(false, is_isomorphic(s,t));

    let s = String::from("paper");
    let t = String::from("title");
    assert_eq!(true, is_isomorphic(s,t));
}
