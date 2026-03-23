//! # Appwrite SDK for Rust
//!
//! Appwrite backend as a service cuts up to 70% of the time and costs required for building a modern application. We abstract and simplify common development tasks behind a REST APIs, to help you develop your app in a fast and secure way. For full API documentation and tutorials go to [https://appwrite.io/docs](https://appwrite.io/docs)
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! appwrite = "0.1.0"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use appwrite::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new()
//!         .set_endpoint("https://cloud.appwrite.io/v1")
//!         .set_project("your-project-id")
//!         .set_key("your-api-key");
//!
//!     // Use the client to make API calls
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod enums;
pub mod error;
pub mod input_file;
pub mod models;
pub mod services;

// Utility modules
pub mod id;
pub mod operator;
pub mod permission;
pub mod query;
pub mod role;

// Re-export commonly used types
pub use client::Client;
pub use error::AppwriteError;
pub use input_file::InputFile;

/// Result type alias for SDK operations
pub type Result<T> = std::result::Result<T, AppwriteError>;

/// SDK version
pub const VERSION: &str = "0.1.0";

/// SDK name
pub const SDK_NAME: &str = "Rust";

/// SDK platform
pub const SDK_PLATFORM: &str = "server";

/// SDK language
pub const SDK_LANGUAGE: &str = "rust";
