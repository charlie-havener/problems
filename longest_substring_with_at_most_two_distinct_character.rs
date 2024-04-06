pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {


    let b = s.as_bytes();
    let mut counts = vec![0;123];
    let mut count = 0;

    let (mut left, mut right) = (0, 0);
    let mut ans = 0;

    while right < s.len() {
        let r = &mut counts[b[right] as usize];
        match r {
            1 => {count += 1; *r += 1;}
            _ => (),
        };
        
        if count <= 2 {
            ans = ans.max(right + 1 - left);
        } else {
            while count > 2 {
                let v = &mut counts[b[left] as usize];
                match v {
                    1 => {*v = 0; count -= 1},
                    x => {*x -= 1;},
                }
                left += 1;
            }
        }
        right += 1;
    }

    return ans as i32;
}

#[test]
fn tests() {
    //let s = String::from("eceba");
    //assert_eq!(3, length_of_longest_substring_two_distinct(s));

    //let s = String::from("ccaabbb");
    //assert_eq!(5, length_of_longest_substring_two_distinct(s));

    let s = String::from("aAbBcCdDeExXyYzZ");
    assert_eq!(2, length_of_longest_substring_two_distinct(s));
}
