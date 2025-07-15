fn is_valid(word: String) -> bool {
    if word.len() < 3 {
        return false;
    }

    let mut has_vowel = false;
    let mut has_cons = false;
    
    for b in word.as_bytes() {
        if [b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U'].contains(b) {
            has_vowel = true;
            continue;
        } else if (*b >= b'a' && *b <= b'z') || (*b >= b'A' && *b <= b'Z') {
            has_cons = true;
            continue;
        } else if *b >= b'0' && *b <= b'9' {
            continue;
        }
        return false;
    }


    return has_vowel && has_cons;
}
