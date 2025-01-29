use std::fmt;

type Name = String;
type Password = String;
type Service = String;
type Copy = bool;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Help(Option<Name>),
    Clear,
    Exit,
    Vault(VaultCommand),
    Panic,
    Analyze(Password),
    Generate(PasswordParameters, Copy)
}

#[derive(Debug, Clone, PartialEq)]
pub enum VaultCommand {
    New(Name),
    Open(Name),
    Close,
    List,
    Show(Option<Service>, bool),
    Add(Service),
    Update(Service, VaultField, String),
    Delete(Service),
    Copy(Service, VaultField),
    Destroy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VaultField {
    Username,
    Password,
}

impl fmt::Display for VaultField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VaultField::Username => write!(f, "username"),
            VaultField::Password => write!(f, "password"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PasswordParameters {
    pub length: usize,
    pub symbols: bool,
    pub avoid_ambiguous: bool,
}