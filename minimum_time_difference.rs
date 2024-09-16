pub fn find_min_difference(time_points: Vec<String>) -> i32 {
    
    let mut minutes: Vec<i32> = Vec::with_capacity(time_points.len());
    for tp in time_points {
        let h = &tp[0..=1];
        let m = &tp[3..=4];
        minutes.push(h.parse::<i32>().unwrap() * 60 + m.parse::<i32>().unwrap());
    }
    minutes.sort_unstable();
    
    let mut ans = i32::MAX;
    
    // check all the pairs
    for pair in minutes.windows(2) {
        let diff = pair[1] - pair[0];
        ans = ans.min(diff);
    }

    // check the last to first diff
    let final_diff = {
        let last_to_end = (24*60) - minutes.last().unwrap();
        let start_to_first = minutes[0];
        start_to_first + last_to_end
    };
    ans = ans.min(final_diff);

    return ans;
}

#[test]
fn tests() {
    let v = vec![String::from("23:59"),String::from("00:00")];
    assert_eq!(1, find_min_difference(v));

    let v = vec![String::from("00:00"),String::from("23:59"),String::from("00:00")];
    assert_eq!(0, find_min_difference(v));

    let v = vec![String::from("00:10"), String::from("00:22"), String::from("23:59")];
    assert_eq!(11, find_min_difference(v));
}
