pub fn compare_version(version1: String, version2: String) -> i32 {
    let mut v1 = version1.split('.');
    let mut v2 = version2.split('.');
    loop {
        match (v1.next(), v2.next()) {
            (Some(a), Some(b)) => {
                let n1: i32 = a.parse().expect("n1 {a:?} is not a number");
                let n2: i32 = b.parse().expect("n2 {b:?} is not a number");
                if n1 < n2 { return -1; }
                if n2 < n1 { return 1; }
            },
            (Some(n), None) => {
                if n.parse::<i32>().expect("n {n:?} is not a number") != 0 {
                    return 1
                }
            },
            (None, Some(n)) => {
                if n.parse::<i32>().expect("n {n:?} is not a number") != 0 {
                    return -1
                }
            },
            (None, None) => return 0,
        }
    }
}

#[test]
fn tests() {
    let s = String::from("1.01");
    let s2 = String::from("1.001");
    assert_eq!(0, compare_version(s,s2));

    let s = String::from("1.0");
    let s2 = String::from("1.0.0");
    assert_eq!(0, compare_version(s,s2));

    let s = String::from("1.0");
    let s2 = String::from("1.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.1");
    assert_eq!(-1, compare_version(s,s2));

    let s = String::from("0.1");
    let s2= String::from("1.1");
    assert_eq!(-1, compare_version(s,s2));

    let s= String::from("1.1");
    let s2 = String::from("0.1");
    assert_eq!(1, compare_version(s,s2));
}
