pub mod cli {
    pub mod stdin;
    pub mod stdout;
    pub mod commands {
        pub mod enums;
        pub mod parser;
        pub mod commands;
    }
}

pub mod utils {
    pub mod vectors;
}

pub mod services {
    pub mod vault {
        pub mod models;
        pub mod crypto;
        pub mod vault;
        pub mod constants;
        pub mod operations;
    }
}
pub mod state;