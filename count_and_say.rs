pub fn count_and_say(n: i32) -> String {

    let mut curr = String::from("1");
    for _ in 1..n {
        println!("{curr:?}");
        let mut next = String::with_capacity(curr.len() * 2);
        let mut s = curr.chars();
        
        let mut prev = s.next().unwrap();
        let mut count = 1;
        while let Some(c) = s.next() {
            if c == prev {
                count += 1;
            } else {
                next.extend(count.to_string().chars());
                next.push(prev);
                prev = c;
                count = 1;
            }
        }
        next.extend(count.to_string().chars());
        next.extend(prev.to_string().chars());

        curr = next;
    }
    
    println!("{curr:?}");
    return curr;
}

#[test]
fn tests() {
    assert_eq!(String::from("1211"), count_and_say(4));
    assert_eq!(String::from("1"), count_and_say(1));
}
