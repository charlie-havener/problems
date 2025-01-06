pub fn min_operations(boxes: String) -> Vec<i32> {

    let mut ans = vec![0; boxes.len()];
    let boxes = boxes.as_bytes();

    let mut ball_count = 0;
    let mut prev_count = 0;

    for idx in 0..boxes.len() {
        prev_count = ball_count + prev_count;
        ans[idx] += prev_count;
        if boxes[idx] == b'1' {
            ball_count += 1;
        }
    }

    ball_count = 0;
    prev_count = 0;

    for idx in (0..boxes.len()).rev() {
        prev_count = ball_count + prev_count;
        if boxes[idx] == b'1' {
            ball_count += 1;
        }
        ans[idx] += prev_count;
    }

    return ans;
}

#[test]
fn tests() {
    let boxes = String::from("110");
    assert_eq!(vec![1,1,3], min_operations(boxes));

    let boxes = String::from("001011");
    assert_eq!(vec![11,8,5,4,3,4], min_operations(boxes));

    let boxes = String::from("001100110011001100110111110110111110000000010");
    assert_eq!(vec![478,455,432,411,392,373,354,337,322,307,292,279,268,257,246,237,230,223,216,211,208,205,204,205,208,213,220,227,236,247,258,271,286,303,322,343,364,385,406,427,448,469,490,511,534], min_operations(boxes));
}
