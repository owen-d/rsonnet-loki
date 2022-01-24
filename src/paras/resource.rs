use derive_more::From;
use k8s_openapi::api::{apps::v1 as apps, core::v1 as core};
use serde::{Deserialize, Serialize};

/// A Resource is an exposed top level k8s API type.
/// These are what we output and send to k8s for management.
#[derive(Clone, Serialize, Deserialize, From)]
pub enum Resource {
    StatefulSet(apps::StatefulSet),
    Deployment(apps::Deployment),
    Service(core::Service),
    ConfigMap(core::ConfigMap),
    // Intended to be unused. This helps prevent unreachable errors by having a nonsense constructor
    // to match against when validate! macros act on all legitimate resource types.
    Nothing,
}
