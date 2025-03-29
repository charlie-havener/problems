pub fn shortest_way(source: String, target: String) -> i32 {

    let source = source.as_bytes();
    let target = target.as_bytes();
    
    let mut ans = 0;
    
    let mut tptr = 0;
    loop {
        ans += 1;
        let start_tptr = tptr;
        for b in source {
            if *b == target[tptr] {
                tptr += 1;
            }
            if tptr == target.len() { return ans }
        }
        if start_tptr == tptr { return -1 }
    }
}

#[test]
fn tests() {
    let source = String::from("abc");
    let target = String::from("abcbc");
    assert_eq!(2, shortest_way(source, target));

    let source = String::from("abc");
    let target = String::from("acdbc");
    assert_eq!(-1, shortest_way(source, target));

    let source = String::from("xyz");
    let target = String::from("xzyxz");
    assert_eq!(3, shortest_way(source, target));

}
