pub fn nearest_palindromic(n: String) -> String {

    let mut ans = "".into();
    let mut min_diff = i32::MAX;

    let palindrome_diff = |s: String| -> i32 {
        todo!();
    };


    let higher = String::from("1");
    higher.extend(n.chars());

    let lower = String::from("9");
    if n.len() <= 2 {
        String::from("9")
    } else {
        String::from("9").extend(&n[2..].chars());
    };
    let inital = n.clone();



    return ans;
}
