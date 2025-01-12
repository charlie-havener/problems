use std::collections::HashSet;

pub fn equal_digit_frequency(s: String) -> i32 {

    let s = s.as_bytes();
    let mut counts: [usize; 10] = [0; 10];
    let (mut lg, mut lg_cnt, mut act_cnt);
    let mut hs: HashSet<&[u8]> = HashSet::new();

    for i in 0..s.len() {
        counts.fill(0);
        lg = 0;
        lg_cnt = 0;
        act_cnt = 0;
        for j in i..s.len() {
            let idx = (s[j] - b'0') as usize;
            if counts[idx] == 0 { act_cnt += 1 }
            counts[idx] += 1;
            if counts[idx] == lg { lg_cnt += 1 }
            if counts[idx] > lg { lg = counts[idx]; lg_cnt = 1 }

            if lg_cnt == act_cnt {
                hs.insert(&s[i..=j]);
            }
        }
    }

    return hs.len() as i32;
}

#[test]
fn tests() {
    let s = String::from("1212");
    assert_eq!(5, equal_digit_frequency(s));

    let s = String::from("12321");
    assert_eq!(9, equal_digit_frequency(s));

    let s = String::from("12321232123212321232123212321232123212321232123212321232123212321232123212321");
    assert_eq!(9, equal_digit_frequency(s));

    let s = String::from("6549871613248794513264597845613216547984542165498979879465461321326457989451231216546579898754651321654987946513265654987546132165498746512316549879546513216548976546124668798454612313524");
    assert_eq!(399, equal_digit_frequency(s));
}
