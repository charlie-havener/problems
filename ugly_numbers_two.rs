pub fn nth_ugly_number(n: i32) -> i32 {

    let mut store: Vec<i64> = Vec::with_capacity(n as usize);
    store.push(1);
    let mut ptr2 = 0;
    let mut ptr3 = 0;
    let mut ptr5 = 0;

    let mut next_vals: (i64,i64,i64) = (store[ptr2]*2, store[ptr3]*3, store[ptr5]*5);
    for _ in 1..n {
        let low = next_vals.0.min(next_vals.1.min(next_vals.2));
        store.push(low);

        if next_vals.0 == low {
            ptr2 += 1;
            next_vals.0 = store[ptr2]*2;
        }
        if next_vals.1 == low {
            ptr3 += 1;
            next_vals.1 = store[ptr3]*3;
        }
        if next_vals.2 == low {
            ptr5 += 1;
            next_vals.2 = store[ptr5]*5;
        }
    }
    return *store.last().unwrap() as i32;
}


#[test]
fn tests() {
    assert_eq!(12, nth_ugly_number(10));
    assert_eq!(1, nth_ugly_number(1));
    assert_eq!(1, nth_ugly_number(1));
    assert_eq!(2123366400, nth_ugly_number(1690));
}
