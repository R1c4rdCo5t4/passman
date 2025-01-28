#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Unknown,
    Help,
    Exit,
    Session(Session),
    Credentials(Credentials),
    Folders(Folders),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Session {
    New,
    Login,
    Status,
    Extend,
    Logout,
    Panic,
    Delete,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Credentials {
    Show(Option<String>, bool),
    Add, // TODO
    Update(String, String),
    Delete(String),
    Search(String),
    Generate, // TODO
    Analyze(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Folders {
    Show(Option<String>),
    Add(String),
    Delete(String),
    Rename(String, String),
}