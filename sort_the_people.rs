pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut t = names.into_iter().zip(heights).collect::<Vec<_>>();
    t.sort_by_key(|&(_,k)| -k);
    return t.into_iter().map(|e| e.0).collect::<Vec<_>>();
}

#[test]
fn tests() {
    let n = vec![String::from("Mary"),String::from("John"),String::from("Emma")];
    let h = vec![180,165,170];
    assert_eq!(vec![String::from("Mary"),String::from("Emma"),String::from("John")], sort_people(n,h));

     
    let n = vec![String::from("Alice"),String::from("Bob"),String::from("Bob")];
    let h = vec![155,185,150];
    assert_eq!(vec![String::from("Bob"),String::from("Alice"),String::from("Bob")], sort_people(n,h));
}
