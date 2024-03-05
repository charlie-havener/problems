use std::cmp::Ordering;

pub fn minimum_length(s: String) -> i32 {
    let b = s.as_bytes();

    println!("{}",b'a');

    let mut ptr1 = 0;
    let mut ptr2 = b.len() - 1;

    loop {
        if ptr1 > ptr2 { return 0 }
        if ptr1 == ptr2 { return 1 }
        match b[ptr1].cmp(&b[ptr2]) {
            Ordering::Less | Ordering::Greater => return (ptr2 - ptr1 + 1) as i32, //todo
            Ordering::Equal => {
                while ptr1 < ptr2 && b[ptr1 + 1] == b[ptr1] {
                    ptr1 += 1;
                }
                while ptr2 > ptr1 && b[ptr2 - 1] == b[ptr2] {
                    ptr2 -= 1;
                }
                ptr1 += 1;
                ptr2 -= 1;
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_tests() {
        let s = String::from("ca");
        assert_eq!(2, minimum_length(s));
        let s = String::from("cabaabac");
        assert_eq!(0, minimum_length(s));
        let s = String::from("aabccabba");
        assert_eq!(3, minimum_length(s));
    }

    #[test]
    fn other_tests() {
        let s = String::from("c");
        assert_eq!(1, minimum_length(s));
        let s = String::from("cc");
        assert_eq!(0, minimum_length(s));
        let s = String::from("ccccccccc");
        assert_eq!(0, minimum_length(s));
        let s = String::from("cac");
        assert_eq!(1, minimum_length(s));
        let s = String::from("caac");
        assert_eq!(0, minimum_length(s));
        let s = String::from("caaac");
        assert_eq!(0, minimum_length(s));
    }
}
