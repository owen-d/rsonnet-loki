use super::pod::{HasPodSpec, HasPodSpecMut};
use k8s_openapi::api::core::v1::{self as core, Affinity, PodSpec};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;

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

pub fn self_anti_affinity<T: HasAffinityMut>(x: T, sel: LabelSelector) -> T {
    let affinity = Affinity {
        pod_anti_affinity: Some(core::PodAntiAffinity {
            required_during_scheduling_ignored_during_execution: Some(vec![
                core::PodAffinityTerm {
                    label_selector: Some(sel),
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
    use k8s_openapi::api::core::v1::{self as core, PodSpec};
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
    use std::collections::BTreeMap;

    #[test]
    fn pod_anti_affinity() {
        let sel = LabelSelector {
            match_labels: Some(BTreeMap::from([(
                "name".to_string(),
                "ingester".to_string(),
            )])),
            ..Default::default()
        };

        let ps: Option<PodSpec> = Some(Default::default());
        let x = super::self_anti_affinity(ps, sel.clone());
        assert_eq!(
            core::PodAffinityTerm {
                label_selector: Some(sel),
                topology_key: super::K8S_HOSTNAME.to_string(),
                ..Default::default()
            },
            x.unwrap()
                .affinity
                .unwrap()
                .pod_anti_affinity
                .unwrap()
                .required_during_scheduling_ignored_during_execution
                .unwrap()
                .get(0)
                .unwrap()
                .clone(),
        );
    }
}
