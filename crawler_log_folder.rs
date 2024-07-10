pub fn min_operations(logs: Vec<String>) -> i32 {

    let mut depth = 0;

    for s in logs {
        match &s[0..=1] {
            ".." => depth = 0.max(depth-1),
            "./" => (),
            _ => depth += 1,
        }
    }
    return depth;
}

#[test]
fn tests() {
    let l = vec![String::from("d1/"),String::from("d2/"),String::from("../"),String::from("d21/"),String::from("./")];
    assert_eq!(2, min_operations(l));
    let l = vec![String::from("d1/"),String::from("d2/"),String::from("./"),String::from("d3/"),String::from("../"),String::from("d31/")];
    assert_eq!(3, min_operations(l));
    let l = vec![String::from("d1/"),String::from("../"),String::from("../"),String::from("../")];
    assert_eq!(0, min_operations(l));
}
