#[derive(Debug, Clone, PartialEq)]
pub struct PasswordParams {
    pub length: usize,
    pub avoid_ambiguous: bool,
}