use crate::domain::cli::field::Field;
use crate::domain::cli::password_params::PasswordParams;

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
    Generate(PasswordParams, Copy)
}

#[derive(Debug, Clone, PartialEq)]
pub enum VaultCommand {
    New(Name),
    Open(Name),
    Close,
    List,
    Show(Option<Service>, bool),
    Add(Service),
    Update(Service, Field, String),
    Delete(Service),
    Copy(Service, Field),
    Destroy,
}

