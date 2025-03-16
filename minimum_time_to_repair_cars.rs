pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {

    let mut ans = i64::MAX;
    let cars = cars as i64;
    let mut left: i64 = 0;
    let mut right: i64 = *ranks.iter().max().unwrap() as i64 * cars as i64 * cars as i64;

    let is_possible = |target: i64| -> bool {
        let mut total = 0;
        for r in &ranks {
            let r = *r as i64;
            total += f64::sqrt((target / r) as f64) as i64pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {

    let mut ans = i64::MAX;
    let cars = cars as i64;
    let mut left: i64 = 0;
    let mut right: i64 = *ranks.iter().max().unwrap() as i64 * cars as i64 * cars as i64;

    let is_possible = |target: i64| -> bool {
        let mut total = 0;
        for r in &ranks {
            let r = *r as i64;
            total += f64::sqrt((target / r) as f64) as i64;
            if total >= cars {
                return true;
            }
        }
        return false;
    };

    while left <= right {
        let target = (right - left) / 2 + left;
        if is_possible(target) {
            right = target - 1;
            ans = ans.min(target);
        } else {
            left = target + 1;
        }
    }

    return ans;
};
            if total >= cars {
                return true;
            }
        }
        return false;
    };

    while left <= right {
        let target = (right - left) / 2 + left;
        if is_possible(target) {
            right = target - 1;
            ans = ans.min(target);
        } else {
            left = target + 1;
        }
    }

    return ans;
}

#[test]
fn tests() {
    let ranks = vec![4,2,3,1];
    let cars = 10;
    assert_eq!(16, repair_cars(ranks, cars));
    
    let ranks = vec![5,1,8];
    let cars = 6;
    assert_eq!(16, repair_cars(ranks, cars));

    
}
