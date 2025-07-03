pub fn kth_character(k: i32) -> char {

    let mut word = vec![b'a'];
    loop {
        let cnt = word.len();
        for idx in 0..cnt {
            if word[idx] == b'z' {
                word.push(b'a');
            } else {
                word.push(word[idx] + 1);
            }
            if word.len() == 500 {
                println!("{word:?}");
                println!("{:?}", String::from_utf8(word).unwrap());
                return 'a'
            }
        }
    }
}

#[test]
fn tests() {
    kth_character(1);
}
