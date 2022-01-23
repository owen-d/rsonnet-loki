use k8s_openapi::api::{
    apps::v1::{Deployment, StatefulSet},
    core::v1::{ConfigMap, Service},
};

pub enum Resource {
    Sts(StatefulSet),
    Deploy(Deployment),
    Svc(Service),
    CfgMap(ConfigMap),
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
