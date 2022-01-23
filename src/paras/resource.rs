use k8s_openapi::api::{apps::v1 as apps, core::v1 as core};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Resource {
    StatefulSet(apps::StatefulSet),
    Deployment(apps::Deployment),
    Service(core::Service),
    ConfigMap(core::ConfigMap),
    // Intended to be unused. This helps prevent unreachable errors by having a nonsense constructor
    // to match against when validate! macros act on all legitimate resource types.
    Nothing,
}

impl From<apps::StatefulSet> for Resource {
    fn from(x: apps::StatefulSet) -> Self {
        Self::StatefulSet(x)
    }
}

impl From<apps::Deployment> for Resource {
    fn from(x: apps::Deployment) -> Self {
        Self::Deployment(x)
    }
}

impl From<core::Service> for Resource {
    fn from(x: core::Service) -> Self {
        Self::Service(x)
    }
}

impl From<core::ConfigMap> for Resource {
    fn from(x: core::ConfigMap) -> Self {
        Self::ConfigMap(x)
    }
}
