pub fn min_max_difference(num: i32) -> i32 {

    // min made by replacing the right most non-zero with zero
    let mut small = num.to_string();
    let mut replace = b'_';
    for c in small.as_bytes() {
        if *c != b'0' {
            replace = *c;
            break;
        }
    }
    unsafe {
        for c in small.as_bytes_mut() {
            if *c == replace {
                *c = 0;
            }
        }
    }
    let small = small.parse::<i32>().unwrap();

    // max made by replacing the right most non-nine with nine
    let mut large = num.to_string();
    let mut replace = b'_';
    for c in large.as_bytes() {
        if *c != b'9' {
            replace = *c;
            break;
        }
    }
    unsafe {
        for c in large.as_bytes_mut() {
            if *c == replace {
                *c = 9;
            }
        }
    }
    let large = large.parse::<i32>().unwrap();


    return large - small;
}
