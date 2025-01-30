#[derive(Debug, Clone, PartialEq)]
pub struct PasswordParams {
    pub length: usize,
    pub symbols: bool,
    pub avoid_ambiguous: bool,
}