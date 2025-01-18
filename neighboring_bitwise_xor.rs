pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
    derived.iter().fold(0, |acc, d| d^acc) == 0
}
