pub fn number_to_words(num: i32) -> String {
    
    let mut ret = Vec::new();

    if num == 0 { return String::from("Zero") }

    let lookups = |n: i32| -> String {
        match n {
            1 => String::from("One"),
            2 => String::from("Two"),
            3 => String::from("Three"),
            4 => String::from("Four"),
            5 => String::from("Five"),
            6 => String::from("Six"),
            7 => String::from("Seven"),
            8 => String::from("Eight"),
            9 => String::from("Nine"),
            10 => String::from("Ten"),
            11 => String::from("Eleven"),
            12 => String::from("Twelve"),
            13 => String::from("Thirteen"),
            14 => String::from("Fourteen"),
            15 => String::from("Fifteen"),
            16 => String::from("Sixteen"),
            17 => String::from("Seventeen"),
            18 => String::from("Eighteen"),
            19 => String::from("Nineteen"),
            20 => String::from("Twenty"),
            30 => String::from("Thirty"),
            40 => String::from("Fourty"),
            50 => String::from("Fifty"),
            60 => String::from("Sixty"),
            70 => String::from("Seventy"),
            80 => String::from("Eighty"),
            90 => String::from("Ninety"),
            _ => unreachable!(),
        }
    };

    let mut curr = num;
    let mut depth = 0;
    while curr != 0 {
        let mut ans = Vec::new();

        let mut segment = curr % 1_000;
        if segment == 0 {
            depth += 1;
            curr /= 1_000;
            continue;
        }

        if segment >= 100 {
            let h = segment / 100;
            ans.push(lookups(h));
            ans.push(String::from("Hundred"));
            segment -= h * 100;
        }

        if segment >= 20 {
            let t = (segment / 10) * 10;
            ans.push(lookups(t));
            segment -= t;
        }

        if segment >= 10 {
            ans.push(lookups(segment));
            segment = 0;
        }

        if segment >= 1 {
            ans.push(lookups(segment));
        }

        match depth {
            1 => ans.push(String::from("Thousand")),
            2 => ans.push(String::from("Million")),
            3 => ans.push(String::from("Billion")),
            _ => (),
        }

        curr /= 1_000;
        depth += 1;
        println!("{:?}", ans);
        ret.push(ans.join(" "));
        println!("{ret:?}");
    }

    ret.reverse();
    return ret.join(" ");

}

#[test]
fn tests() {
    let n = 123;
    assert_eq!(String::from("One Hundred Twenty Three"), number_to_words(n));

    let n = 12345;
    assert_eq!(String::from("Twelve Thousand Three Hundred Fourty Five"), number_to_words(n));

    let n = 1234567;
    assert_eq!(String::from("One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"), number_to_words(n));

    let n = 0;
    assert_eq!(String::from("Zero"), number_to_words(n));

    let n = 1_000_000_000;
    assert_eq!(String::from("One Billion"), number_to_words(n));

    let n = 1_000_000;
    assert_eq!(String::from("One Million"), number_to_words(n));

    let n = 1_000;
    assert_eq!(String::from("One Thousand"), number_to_words(n));

    let n = 100;
    assert_eq!(String::from("One Hundred"), number_to_words(n));

    let n = 1_060_900_012;
    assert_eq!(String::from("One Billion Sixty Million Nine Hundred Thousand Twelve"), number_to_words(n));
}
