pub mod cli {
    pub mod stdin;
    pub mod stdout;
    pub mod commands {
        pub mod models;
        pub mod parser;
        pub mod commands;
    }
}

pub mod utils {
    pub mod vectors;
}

pub mod services {
    pub mod error;

    pub mod vault {
        pub mod models {
            pub mod vault;
            pub mod vault_file;
            pub mod password_entry;
        }
        pub mod crypto;
        pub mod vault;
        pub mod constants;
        pub mod operations;
    }
}
pub mod state;