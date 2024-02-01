pub fn generate_possible_next_moves(current_state: String) -> Vec<String> {
    let mut ans = vec![];

    for (idx, &v) in current_state[0..current_state.len() - 1].as_bytes().iter().enumerate() {
        if current_state.as_bytes()[idx] == current_state.as_bytes()[idx + 1] && v == b'+' {
            let mut t = current_state.clone();
            // SAFETY: the string consists of only the '+' and '-' chars
            // and we are only replacing chars with the '+' or '-' char
            // therefore we can guarantee we have valid utf-8
            unsafe {
                t.as_bytes_mut()[idx] = b'-';
                t.as_bytes_mut()[idx + 1] = b'-';
            }
            ans.push(t);
        }
    }
    return ans;
}

#[cfg(test)]
mod test {
    use super::generate_possible_next_moves;

    #[test]
    fn test() {
        assert_eq!(vec![String::from("--++"),String::from("+--+"),String::from("++--")], generate_possible_next_moves(String::from("++++")));
        assert_eq!(vec![] as Vec<String>, generate_possible_next_moves(String::from("+")));
    }
}
