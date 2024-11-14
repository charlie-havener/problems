pub fn minimized_maximum(n: i32, mut quantities: Vec<i32>) -> i32 {

    quantities.sort_unstable();

    let is_valid = |val: i32| -> bool {
        
        let mut stores_left = n;

        // any quantity <= val only needs a single store.
        // bsearch for the first quantity greater than val (that index will be stored in right)
        let (mut left, mut right): (i32, i32) = (-1, quantities.len() as i32);
        while right - left > 1 {
            let mid = left + (right - left) / 2;
            if val < quantities[mid as usize] { right = mid }
            else { left = mid }
        }

        stores_left -= right;
        if stores_left < 0 { return false }

        for i in right as usize..quantities.len() {
            let mut q = quantities[i] / val;
            if q * val != quantities[i] { q += 1 }
            stores_left -= q;
            if stores_left < 0 { return false }
        }

        return true;
    };


    // bsearch over the solution space

    let (mut left, mut right) = (1, 100_000);
    let mut ans = i32::MAX;

    while left <= right {
        let mid = left + (right - left) / 2;
        
        if is_valid(mid) {
            ans = ans.min(mid);
            right = mid - 1;
        }
        else {
            left = mid + 1;
        }
    }
    return ans;
}

#[test]
fn tests() {
    let n = 6;
    let quantities = vec![11,6];
    assert_eq!(3, minimized_maximum(n, quantities));

    let n = 7;
    let quantities = vec![15,10,10];
    assert_eq!(5, minimized_maximum(n, quantities));

    let n = 1;
    let quantities = vec![100_000];
    assert_eq!(100_000, minimized_maximum(n, quantities));

    let n = 2;
    let quantities = vec![100_000];
    assert_eq!(50_000, minimized_maximum(n, quantities));
}
