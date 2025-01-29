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
    Copy(VaultField),
    Search(String),
    Destroy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VaultField {
    Username,
    Password,
}
