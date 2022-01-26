#[allow(unused_imports)]
#[macro_use] extern crate log;
extern crate yaml_rust;

#[allow(unused_imports)]
use kube::{
    CustomResource, CustomResourceExt,
};

#[path = "../crd.rs"] mod crd;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=info");
    env_logger::init();

    let crd_yaml: String = serde_yaml::to_string(&crd::SecretGenerator::crd())?;    
    info!("YAML generated:\n{}", crd_yaml);
    Ok(())
}
