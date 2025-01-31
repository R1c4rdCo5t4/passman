pub mod cli {
    pub mod commands;
    pub mod parser;
    pub mod io;
}

pub mod domain {
    pub mod cli {
        pub mod commands;
        pub mod field;
        pub mod password_params;
    }
    pub mod vault {
        pub mod vault;
        pub mod vault_file;
        pub mod password_entry;
    }
    pub mod app {
        pub mod state;
        pub mod session;
        pub mod error;
    }
}

pub mod services {
    pub mod vault_service;
}

pub mod repository {
    pub mod vault {
        pub mod vault_crypto;
        pub mod vault_manager;
        pub mod vault_manager_trait;
    }
}

pub mod utils {
    pub mod constants;
    pub mod validation;
    pub mod passwords;
}
