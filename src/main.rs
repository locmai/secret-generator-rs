use futures::stream::StreamExt;
use kube::Resource;
use kube::ResourceExt;
use kube::{api::ListParams, client::Client, Api};
use kube_runtime::controller::{Context, ReconcilerAction};
use kube_runtime::Controller;
use tokio::time::Duration;

use crate::crd::SecretGenerator;

pub mod crd;
mod finalizer;
mod secretgenerator;

#[tokio::main]
async fn main() {
    // First, a Kubernetes client must be obtained using the `kube` crate
    // The client will later be moved to the custom controller
    let kubernetes_client: Client = Client::try_default()
        .await
        .expect("Expected a valid KUBECONFIG environment variable.");

    // Preparation of resources used by the `kube_runtime::Controller`
    let crd_api: Api<SecretGenerator> = Api::all(kubernetes_client.clone());
    let context: Context<ContextData> = Context::new(ContextData::new(kubernetes_client.clone()));

    Controller::new(crd_api.clone(), ListParams::default())
        .run(reconcile, on_error, context)
        .for_each(|reconciliation_result| async move {
            match reconciliation_result {
                Ok(secretgenerator_resource) => {
                    println!("Reconciliation successful. Resource: {:?}", secretgenerator_resource);
                }
                Err(reconciliation_err) => {
                    eprintln!("Reconciliation error: {:?}", reconciliation_err)
                }
            }
        })
        .await;
}

/// Context injected with each `reconcile` and `on_error` method invocation.
struct ContextData {
    /// Kubernetes client to make Kubernetes API requests with. Required for K8S resource management.
    client: Client,
}

impl ContextData {
    /// Constructs a new instance of ContextData.
    ///
    /// # Arguments:
    /// - `client`: A Kubernetes client to make Kubernetes REST API requests with. Resources
    /// will be created and deleted with this client.
    pub fn new(client: Client) -> Self {
        ContextData { client }
    }
}

enum Action {
    /// Create the subresources, this includes spawning `n` pods with Echo service
    Create,
    /// Delete all subresources created in the `Create` phase
    Delete,
    /// This `Echo` resource is in desired state and requires no actions to be taken
    NoOp,
}

async fn reconcile(secret_generator: SecretGenerator, context: Context<ContextData>) -> Result<ReconcilerAction, Error> {
    let client: Client = context.get_ref().client.clone(); // The `Client` is shared -> a clone from the reference is obtained

    let namespace: String = match secret_generator.namespace() {
        None => {
            // If there is no namespace to deploy to defined, reconciliation ends with an error immediately.
            return Err(Error::UserInputError(
                "Expected SecretGenerator resource to be namespaced. Can't deploy to an unknown namespace."
                    .to_owned(),
            ));
        }
        // If namespace is known, proceed. In a more advanced version of the operator, perhaps
        // the namespace could be checked for existence first.
        Some(namespace) => namespace,
    };

    // Performs action as decided by the `determine_action` function.
    return match determine_action(&secret_generator) {
        Action::Create => {
            let name = secret_generator.name();

            finalizer::add(client.clone(), &name, &namespace).await?;
            secretgenerator::deploy(client, &secret_generator.name(), secret_generator.spec.secrets, &namespace).await?;
            Ok(ReconcilerAction {
                // Finalizer is added, deployment is deployed, re-check in 10 seconds.
                requeue_after: Some(Duration::from_secs(10)),
            })
        }
        Action::Delete => {
            secretgenerator::delete(client.clone(), &secret_generator.name(), &namespace).await?;

            finalizer::delete(client, &secret_generator.name(), &namespace).await?;
            Ok(ReconcilerAction {
                requeue_after: None, // Makes no sense to delete after a successful delete, as the resource is gone
            })
        }
        Action::NoOp => Ok(ReconcilerAction {
            requeue_after: Some(Duration::from_secs(10)),
        }),
    };
}

fn determine_action(secret_generator: &SecretGenerator) -> Action {
    return if secret_generator.meta().deletion_timestamp.is_some() {
        Action::Delete
    } else if secret_generator
        .meta()
        .finalizers
        .as_ref()
        .map_or(true, |finalizers| finalizers.is_empty())
    {
        Action::Create
    } else {
        Action::NoOp
    };
}

/// Actions to be taken when a reconciliation fails - for whatever reason.
/// Prints out the error to `stderr` and requeues the resource for another reconciliation after
/// five seconds.
///
/// # Arguments
/// - `error`: A reference to the `kube::Error` that occurred during reconciliation.
/// - `_context`: Unused argument. Context Data "injected" automatically by kube-rs.
fn on_error(error: &Error, _context: Context<ContextData>) -> ReconcilerAction {
    eprintln!("Reconciliation error:\n{:?}", error);
    ReconcilerAction {
        requeue_after: Some(Duration::from_secs(5)),
    }
}

/// All errors possible to occur during reconciliation
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Any error originating from the `kube-rs` crate
    #[error("Kubernetes reported error: {source}")]
    KubeError {
        #[from]
        source: kube::Error,
    },
    /// Error in user input or Echo resource definition, typically missing fields.
    #[error("Invalid CRD: {0}")]
    UserInputError(String),
}
