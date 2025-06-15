pub fn max_diff(num: i32) -> i32 {

    //smallest = replace left most non zero with a zero
    //cannot replace the first char with a zero, but if the first char
    //is not one then make it a one
    let mut small = num.to_string();
    let mut rep = b'_';
    let mut to = b'_';
    let s = small.as_bytes();
    if s[0] != b'1' {
        rep = s[0];
        to = b'1';
    }
    else {
        for c in s {
            if *c != b'0' && *c != s[0] {
                rep = *c;
                to = b'0';
                break;
            }
        }
    }
    if rep != b'_' {
        unsafe {
            for c in small.as_bytes_mut() {
                if *c == rep {
                    *c = to;
                }
            }
        }
    }


    // largest = replace left most non nine with nine
    let mut large = num.to_string();
    let mut rep = b'_';
    for c in large.as_bytes() {
        if *c != b'9' {
            rep = *c;
            break;
        }
    }
    if rep != b'_' {
        unsafe {
            for c in large.as_bytes_mut() {
                if *c == rep {
                    *c = b'9';
                }
            }
        }
    }

    return large.parse::<i32>().unwrap() - small.parse::<i32>().unwrap();
}
