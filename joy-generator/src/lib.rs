//! Joy Goodbye - Generate warm, heartfelt sign-off messages
//!
//! This is the core Rust library. Language bindings (Python, TypeScript) wrap this library.

pub mod corpus;
pub mod datetime;
pub mod emoji;
pub mod error;
pub mod generate;
pub mod options;
pub mod template;

pub use error::GoodbyeError;
pub use options::CoreGoodbyeOptions;

// Re-export the main function
pub use generate::generate_goodbye;
