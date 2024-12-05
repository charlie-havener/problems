pub fn can_change(start: String, target: String) -> bool {

    let mut start = start.chars().enumerate().peekable();
    let mut target = target.chars().enumerate().peekable();

    let mut ptr_s = (0, '_');
    let mut ptr_t = (0, '_');
    loop {
        
        // get the next letter in start
        while let Some((idx, c)) = start.next() {
            match c {
                '_' => continue,
                'L' => { ptr_s = (idx, 'L'); break },
                'R' => { ptr_s = (idx, 'R'); break },
                _ => unreachable!(),
            }
        }
        
        // get the next letter in target
        while let Some((idx, c)) = target.next() {
            match c {
                '_' => continue,
                'L' => { ptr_t = (idx, 'L'); break },
                'R' => { ptr_t = (idx, 'R'); break },
                _ => unreachable!(),
            }
        }
        
        // make sure it's valid
        if ptr_s.1 != ptr_t.1 { return false }
        else if ptr_s.1 == 'L' && ptr_s.0 < ptr_t.0 { return false }
        else if ptr_s.1 == 'R' && ptr_s.0 > ptr_t.0 { return false }
        ptr_s = (0, '_');
        ptr_t = (0, '_');
        if start.peek().is_none() && target.peek().is_none() { return true }
    }

}

#[test]
fn tests() {
    let start = String::from("_L__R__R_");
    let target = String::from("L______RR");
    assert_eq!(true, can_change(start, target));

    let start = String::from("R_L_");
    let target = String::from("__LR");
    assert_eq!(false, can_change(start, target));

    let start = String::from("_R");
    let target = String::from("R_");
    assert_eq!(false, can_change(start, target));

    let start = String::from("_LL__R__R_");
    let target = String::from("L___L___RR");
    assert_eq!(false, can_change(start, target));

    let start = String::from("____");
    let target = String::from("R_L_");
    assert_eq!(false, can_change(start, target));

    let start = String::from("_L");
    let target = String::from("LL");
    assert_eq!(false, can_change(start, target));
}
