use k8s_openapi::api::{
    apps::v1::{Deployment, StatefulSet},
    core::v1::{ConfigMap, Service},
};

#[derive(Clone)]
pub enum Resource {
    Sts(StatefulSet),
    Deploy(Deployment),
    Svc(Service),
    CfgMap(ConfigMap),
    // Intended to be unused. This helps prevent unreachable errors by having a nonsense constructor
    // to match against when validate! macros act on all legitimate resource types.
    Nothing,
}

impl From<StatefulSet> for Resource {
    fn from(x: StatefulSet) -> Self {
        Self::Sts(x)
    }
}

impl From<Deployment> for Resource {
    fn from(x: Deployment) -> Self {
        Self::Deploy(x)
    }
}

impl From<Service> for Resource {
    fn from(x: Service) -> Self {
        Self::Svc(x)
    }
}

impl From<ConfigMap> for Resource {
    fn from(x: ConfigMap) -> Self {
        Self::CfgMap(x)
    }
}
