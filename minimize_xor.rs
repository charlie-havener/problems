pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
    let n1 = num1.count_ones() as i32;
    let n2 = num2.count_ones() as i32;
    let mut diff = (n1 - n2).abs();

    let mut ans = num1;


    if n1 == n2 {
        return ans;
    }
    // need to add 1
    else if n1 < n2 {
        for s in 0..32 {
            if num1 ^ (1<<s) > num1 {
                ans |= 1<<s;
                diff -= 1;
            }
            if diff == 0 { return ans }
        }
    }
    // need to remove 1s
    else {
        for s in 0..32 {
            if num1 & (1<<s) > 0 {
                ans ^= 1<<s;
                diff -= 1;
            }
            if diff == 0 { return ans }
        }
    }

    return ans;
}

#[test]
fn tests() {
    let num1 = 3;
    let num2 = 5;
    assert_eq!(3, minimize_xor(num1, num2));

    let num1 = 1;
    let num2 = 12;
    assert_eq!(3, minimize_xor(num1, num2));

    let num1 = 27;
    let num2 = 5;
    assert_eq!(24, minimize_xor(num1, num2));

    let num1 = 1;
    let num2 = 3;
    assert_eq!(3, minimize_xor(num1, num2));

    let num1 = 954978452;
    let num2 = 974526854;
    println!("{num1:b}");
    println!("{num2:b}");
    println!("{:b}", 954978432);
    println!("{:b}", 954978455);
    assert_eq!(954978432, minimize_xor(num1, num2));
}
