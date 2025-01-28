
pub fn get_str(vec: &Vec<&str>, index: usize) -> String {
    vec.get(index).unwrap().to_string()
}

pub fn get_opt_str(vec: &Vec<&str>, index: usize) -> Option<String> {
    vec.get(index).map(|s| (*s).to_string())

}