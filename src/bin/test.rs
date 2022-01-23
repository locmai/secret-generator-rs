#[allow(unused_imports)]
#[macro_use] extern crate log;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

extern crate byte_string;
extern crate base64;

use k8s_openapi::ByteString;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let gen_secret: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
    println!("vcl");
    println!("dd {}", gen_secret);


    let bs = base64::encode(&gen_secret);
    println!("asdasdsad");

    println!("type {}", &bs);

    let bs2 = ByteString::try((&bs.as_bytes()).to_vec());
    println!("{:?}", &bs2);

    Ok(())
}
