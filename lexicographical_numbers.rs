pub fn lexical_order(n: i32) -> Vec<i32> {
    
    let mut ans: Vec<i32> = Vec::with_capacity(n as usize);
    
    let mut curr = 1;

    for _ in 0..n {
        ans.push(curr);
        if curr*10 <= n { curr *= 10 }
        else {
            while curr%10 == 9 || curr >= n {
                curr /= 10;
            }
            curr += 1;
        }
    }

    return ans;
}

#[test]
fn tests() {
    println!("{:?}", lexical_order(13));
    println!("{:?}", lexical_order(2));
    println!("{:?}", lexical_order(112));
    //println!("{:?}", lexical_order(1112));
}
