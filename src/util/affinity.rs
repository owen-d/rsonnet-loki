use super::conventions::{Has, HasMut};
use k8s_openapi::api::core::v1::{self as core, Affinity, PodSpec};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;

pub const K8S_HOSTNAME: &str = "kubernetes.io/hostname";

impl<T: Has<PodSpec>> Has<Affinity> for T {
    fn get(&self) -> Option<Affinity> {
        self.get().and_then(|x| x.affinity)
    }
}

impl<T: HasMut<PodSpec>> HasMut<Affinity> for T {
    fn with(&self, x: Affinity) -> Self {
        let spec = PodSpec {
            affinity: Some(x),
            ..self.get().clone().unwrap_or_default()
        };
        self.with(spec)
    }
}

pub fn self_anti_affinity<T: HasMut<Affinity>>(x: T, sel: LabelSelector) -> T {
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

    x.with(affinity)
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
