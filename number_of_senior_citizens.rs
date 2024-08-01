pub fn count_seniors(details: Vec<String>) -> i32 {
    
    let mut ans = 0;
    for d in details {
        let age = &d[11..=12].parse::<i32>().unwrap();
        if *age > 60 { ans += 1 }
    }
    return ans;
}

#[test]
fn tests() {
    let d = vec![String::from("7868190130M7522"),String::from("5303914400F9211"),String::from("9273338290F4010")];
    assert_eq!(2, count_seniors(d));

    let d = vec![String::from("1313579440F2036"),String::from("2921522980M5644")];
    assert_eq!(0, count_seniors(d));
}
