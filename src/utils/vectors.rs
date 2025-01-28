
pub fn get_str(vec: &Vec<&str>, index: usize) -> Option<String> {
    vec.get(index).map(|s| (*s).to_string())
}