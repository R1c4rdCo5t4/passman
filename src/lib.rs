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