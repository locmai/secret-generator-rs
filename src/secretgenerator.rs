extern crate base64;
extern crate byte_string;
use k8s_openapi::ByteString;

use k8s_openapi::api::core::v1::{Secret};
use kube::api::{DeleteParams, ObjectMeta, PostParams};
use kube::{Api, Client, Error};
use std::collections::BTreeMap;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use crate::crd::DeclaredSecretSpec;


/// Creates a new secret with the given secrets
/// # Arguments
/// - `client` - A Kubernetes client to create the deployment with.
/// - `name` - Name of the deployment to be created
/// - `replicas` - Number of pod replicas for the Deployment to contain
/// - `namespace` - Namespace to create the Kubernetes Deployment in.
///
/// Note: It is assumed the resource does not already exists for simplicity. Returns an `Error` if it does.
pub async fn deploy(
    client: Client,
    name: &str,
    secrets: Vec<DeclaredSecretSpec>,
    namespace: &str,
) -> Result<Secret, Error> {
    let mut labels: BTreeMap<String, String> = BTreeMap::new();
    labels.insert("app".to_owned(), name.to_owned());

    let mut secrets_data: BTreeMap<String, ByteString> = BTreeMap::new();

    for secret in secrets {
        let gen_secret: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(secret.lenght.try_into().unwrap())
            .map(char::from)
            .collect();

        let gen_secret_base64_byte_string = ByteString((&gen_secret.as_bytes()).to_vec());
        secrets_data.insert(secret.name.try_into().unwrap(), gen_secret_base64_byte_string);
    };

    let secret: Secret = Secret {
        metadata: ObjectMeta {
            name: Some(name.to_owned()),
            namespace: Some(namespace.to_owned()),
            labels: Some(labels.clone()),
            ..ObjectMeta::default()
            },
        data: Some(secrets_data.clone()),
        ..Default::default()
    };

    // Create the deployment defined above
    let secret_api: Api<Secret> = Api::namespaced(client, namespace);
    secret_api
        .create(&PostParams::default(), &secret)
        .await
}

pub async fn delete(client: Client, name: &str, namespace: &str) -> Result<(), Error> {
    let api: Api<Secret> = Api::namespaced(client, namespace);
    api.delete(name, &DeleteParams::default()).await?;
    Ok(())
}
