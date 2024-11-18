pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    
    let n = code.len() as i32;

    let w = k.abs();
    let mut ans = vec![0; code.len()];

    if k == 0 { return ans }
    

    let mut sum = 0;
    let mut ptr = w;
    for i in 0..w { 
        sum += code[(i+1) as usize];
    }

    loop {
        
        if k > 0 {
            ans[(ptr-w).rem_euclid(n) as usize] = sum;
        } else {
            ans[(ptr+1).rem_euclid(n) as usize] = sum;
        }

        sum -= code[(ptr-w+1).rem_euclid(n) as usize];
        sum += code[(ptr+1).rem_euclid(n) as usize];


        ptr = (ptr+1).rem_euclid(n);
        if ptr == w { break }
    }

    return ans;
}

#[test]
fn tests() {
    let code = vec![5,7,1,4];
    let k = 3;
    assert_eq!(vec![12,10,16,13], decrypt(code, k));

    let code = vec![1,2,3,4];
    let k = 0;
    assert_eq!(vec![0,0,0,0], decrypt(code, k));
    
    let code = vec![2,4,9,3];
    let k = -2;
    assert_eq!(vec![12,5,6,13], decrypt(code, k));
}
