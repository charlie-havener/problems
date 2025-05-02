pub fn push_dominoes(dominoes: String) -> String {

    let dominoes = dominoes.as_bytes();
    let mut v = vec![0; dominoes.len()];

    let mut k = 0;
    for i in 0..dominoes.len() {
        if dominoes[i] == 'R' as u8 { k = dominoes.len() as i32 }
        else if dominoes[i] == 'L' as u8 { k = 0 }
        else { k = 0.max(k-1) }
        v[i] = k;
    }

    k = 0;
    for i in (0..dominoes.len()).rev() {
        if dominoes[i] == 'L' as u8 { k = dominoes.len() as i32 }
        else if dominoes[i] == 'R' as u8 { k = 0 }
        else { k = 0.max(k-1) }

        if v[i] > k { v[i] = 'R' as i32 }
        else if v[i] < k { v[i] = 'L' as i32 }
        else { v[i] = '.' as i32 }
    }


    return v.into_iter().map(|c| (c as u8) as char).collect::<String>();
}
