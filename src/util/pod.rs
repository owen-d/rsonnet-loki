use k8s_openapi::api::apps::v1::DeploymentSpec;
use k8s_openapi::api::core::v1::{PodSpec, PodTemplateSpec};

pub trait HasPodSpec {
    fn pod_spec(&self) -> Option<PodSpec>;
}

pub trait HasPodSpecMut: Clone + HasPodSpec {
    fn with_pod_spec(self, podspec: PodSpec) -> Self;
}

impl HasPodSpec for Option<PodSpec> {
    fn pod_spec(&self) -> Option<PodSpec> {
        self.clone()
    }
}

impl HasPodSpecMut for Option<PodSpec> {
    fn with_pod_spec(self, ps: PodSpec) -> Self {
        Some(ps)
    }
}

impl HasPodSpec for DeploymentSpec {
    fn pod_spec(&self) -> Option<PodSpec> {
        self.template.spec.clone()
    }
}

impl HasPodSpecMut for DeploymentSpec {
    fn with_pod_spec(self, ps: PodSpec) -> Self {
        DeploymentSpec {
            template: PodTemplateSpec {
                spec: Some(ps),
                ..self.template.clone()
            },
            ..self.clone()
        }
    }
}
