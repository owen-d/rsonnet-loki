use super::pod::{HasPodSpec, HasPodSpecMut};
use k8s_openapi::api::core::v1::{Affinity, PodSpec};

pub trait HasAffinity {
    fn affinity(&self) -> Option<Affinity>;
}

pub trait HasAffinityMut: HasAffinity {
    fn with_affinity(&mut self, affinity: Affinity);
}

impl<T: HasPodSpec> HasAffinity for T {
    fn affinity(&self) -> Option<Affinity> {
        self.pod_spec().and_then(|x| x.affinity)
    }
}

impl<T: HasPodSpecMut> HasAffinityMut for T {
    fn with_affinity(&mut self, affinity: Affinity) {
        self.with_pod_spec(PodSpec {
            affinity: Some(affinity),
            ..self.pod_spec().unwrap_or_default()
        })
    }
}
