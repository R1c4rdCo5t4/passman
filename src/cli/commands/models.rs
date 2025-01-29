use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Help(Option<String>),
    Clear,
    Exit,
    Vault(VaultCommand),
    Analyze(String),
    Generate, // TODO
}

#[derive(Debug, Clone, PartialEq)]
pub enum VaultCommand {
    New(String),
    Open(String),
    Close,
    List,
    Show(Option<String>, bool),
    Add(String, String, String),
    Update(String, VaultField, String),
    Delete(String),
    Copy(String, VaultField),
    Panic,
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