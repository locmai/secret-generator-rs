use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
// use chrono::{DateTime, Utc};
#[allow(unused_imports)]
use apiexts::CustomResourceDefinition;
#[allow(unused_imports)]
use 
k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1 as apiexts;
#[allow(unused_imports)]
use kube::{
    api::{Api, Patch, PatchParams, ResourceExt},
    runtime::wait::{await_condition, conditions},
    Client, CustomResource, CustomResourceExt,
};

fn default_backend_value() -> String {
    "Kubernetes".into()
}

fn default_status_value() -> String {
    "Not ready".into()
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct DeclaredSecretSpec {
    pub name: String,
    pub length: i32,
}

// SecretGenerator resource specification
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "locmai.dev",
    version = "v1alpha1",
    kind = "SecretGenerator",
    plural = "secretgenerators",
    namespaced,
    status = "SecretGeneratorStatus",
    category = "secretgenerator",
    shortname = "sg",
    printcolumn = r#"{"name":"Condition", "type":"string", "description":"condition of the secret-generator", "jsonPath":".status.condition"}, {"name":"Last updated", "type":"date", "description":"last updated timestamp of the secret-generator", "jsonPath":".status.last_updated"}"#
)]
pub struct SecretGeneratorSpec {
    pub name: String,

    pub secrets: Vec<DeclaredSecretSpec>,
    #[serde(default = "default_backend_value")]
    pub backend: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct SecretGeneratorStatus {
    #[serde(default = "default_status_value")]
    pub condition: String,

    // #[schemars(schema_with = "set_datetime_schema")]
    // pub last_updated: Option<DateTime<Utc>>
}
