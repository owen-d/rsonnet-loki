use super::pod::{HasPodSpec, HasPodSpecMut};
use k8s_openapi::api::core::v1::{self as core, Affinity, PodSpec};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use std::collections::BTreeMap;

pub const K8S_HOSTNAME: &str = "kubernetes.io/hostname";

pub trait HasAffinity {
    fn affinity(&self) -> Option<Affinity>;
}

pub trait HasAffinityMut: Clone + HasAffinity {
    fn with_affinity(self, affinity: Affinity) -> Self;
}

impl<T: HasPodSpec> HasAffinity for T {
    fn affinity(&self) -> Option<Affinity> {
        self.pod_spec().and_then(|x| x.affinity)
    }
}

impl<T: HasPodSpecMut> HasAffinityMut for T {
    fn with_affinity(self, affinity: Affinity) -> Self {
        let spec = PodSpec {
            affinity: Some(affinity),
            ..self.pod_spec().clone().unwrap_or_default()
        };
        self.with_pod_spec(spec)
    }
}

pub fn self_anti_affinity<T: HasAffinityMut>(x: T) -> T {
    let affinity = Affinity {
        pod_anti_affinity: Some(core::PodAntiAffinity {
            required_during_scheduling_ignored_during_execution: Some(vec![
                core::PodAffinityTerm {
                    label_selector: Some(LabelSelector {
                        match_labels: Some(BTreeMap::from([(
                            "name".to_string(),
                            "ingester".to_string(),
                        )])),
                        ..Default::default()
                    }),
                    topology_key: K8S_HOSTNAME.to_string(),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        }),
        ..Default::default()
    };

    x.with_affinity(affinity)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
