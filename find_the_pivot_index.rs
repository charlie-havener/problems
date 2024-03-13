pub fn pivot_index(n: i32) -> i32 {
    let v = f32::sqrt((n*(n+1)) as f32 / 2.0);
    if v.trunc() == v { return v as i32 } else { return -1 }
}

#[test]
fn test() {
    println!("{}", pivot_index(1));
    println!("{}", pivot_index(2));
    println!("{}", pivot_index(3));
    println!("{}", pivot_index(4));
    println!("{}", pivot_index(5));
    println!("{}", pivot_index(6));
    println!("{}", pivot_index(7));
    println!("{}", pivot_index(8));
}
