pub extern crate chrono;
pub extern crate url;
pub use crate::client::error::Error;
pub use crate::client::error::Result;
pub use crate::client::VaultClient as Client;

struct VaultBackend {
    vault_addr: &str,
    vault_token: Option<&str>,
}

