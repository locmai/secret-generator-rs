pub extern crate chrono;
pub extern crate url;
use hashicorp_vault::client::error::Error;
use hashicorp_vault::client::error::Result;
use hashicorp_vault::client::VaultClient;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=info");
    env_logger::init();

    let host = "http://localhost:43765";
    let token = "...";
    let client = VaultClient::new(host, token).unwrap();
    let _ = client.set_secret("foo", "bar");

    let secret = client.get_secret("foo").unwrap();
    Ok(())
}
