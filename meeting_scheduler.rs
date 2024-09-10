pub fn min_available_duration(mut slots1: Vec<Vec<i32>>, mut slots2: Vec<Vec<i32>>, duration: i32) -> Vec<i32> {

    slots1.sort();
    slots2.sort();

    let mut s1 = 0;
    let mut s2 = 0;

    while s1 < slots1.len() && s2 < slots2.len() {
        
        let earliest_start = slots1[s1][0].max(slots2[s2][0]);
        let end_time = earliest_start + duration;

        if end_time <= slots1[s1][1] && end_time <= slots2[s2][1] {
            return vec![earliest_start, end_time];
        }
        if slots1[s1][1] < slots2[s2][1] {
            s1 += 1;
        } else {
            s2 += 1;
        }
    }
    return vec![];
}

#[test]
fn tests() {
    let slots1 = vec![vec![10,50],vec![60,120],vec![140,210]];
    let slots2 = vec![vec![0,15],vec![60,70]];
    let duration = 8;
    assert_eq!(vec![60,68], min_available_duration(slots1, slots2, duration));

    let slots1 = vec![vec![10,50],vec![60,120],vec![140,210]];
    let slots2 = vec![vec![0,15],vec![60,70]];
    let duration = 12;
    assert_eq!(Vec::<i32>::new(), min_available_duration(slots1, slots2, duration));

    let slots1 = vec![vec![216397070,363167701],vec![98730764,158208909],vec![441003187,466254040],vec![558239978,678368334],vec![683942980,717766451]];
    let slots2 = vec![vec![50490609,222653186],vec![512711631,670791418],vec![730229023,802410205],vec![812553104,891266775],vec![230032010,399152578]];
    let duration = 456085;
    assert_eq!(Vec::<i32>::new(), min_available_duration(slots1, slots2, duration));
}
