use k8s_openapi::api::apps::v1::DeploymentSpec;
use k8s_openapi::api::core::v1::PodSpec;

pub trait HasPodSpec {
    fn pod_spec(&self) -> Option<PodSpec>;
}

pub trait HasPodSpecMut: HasPodSpec {
    fn with_pod_spec(&mut self, podspec: PodSpec);
}

impl HasPodSpec for Option<PodSpec> {
    fn pod_spec(&self) -> Option<PodSpec> {
        self.clone()
    }
}

impl HasPodSpecMut for Option<PodSpec> {
    fn with_pod_spec(&mut self, ps: PodSpec) {
        self.replace(ps);
    }
}

impl HasPodSpec for DeploymentSpec {
    fn pod_spec(&self) -> Option<PodSpec> {
        self.template.spec.clone()
    }
}

impl HasPodSpecMut for DeploymentSpec {
    fn with_pod_spec(&mut self, ps: PodSpec) {
        self.template.spec.replace(ps);
    }
}
