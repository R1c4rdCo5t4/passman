#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Help(Option<String>),
    Clear,
    Exit,
    Vault(Vault),
    Analyze(String),
    Generate, // TODO
}

#[derive(Debug, Clone, PartialEq)]
pub enum Vault {
    New(String),
    Open(String),
    Close,
    List,
    Show(Option<String>, bool),
    Add(String, String, String),
    Update(String, String, String),
    Delete(String),
    Copy(String),
    Search(String),
    Destroy,
}

