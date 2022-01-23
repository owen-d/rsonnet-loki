use std::collections::BTreeMap;

use super::conventions::{Has, Name, With};
use k8s_openapi::api::core::v1::{self as core, Affinity, PodSpec};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;

pub const K8S_HOSTNAME: &str = "kubernetes.io/hostname";

impl<T: Has<PodSpec>> Has<Affinity> for T {
    fn get(&self) -> Option<Affinity> {
        self.get().and_then(|x| x.affinity)
    }
}

impl<T: With<PodSpec>> With<Affinity> for T {
    fn with(&self, x: Affinity) -> Self {
        let spec = PodSpec {
            affinity: Some(x),
            ..self.get().unwrap_or_default()
        };
        self.with(spec)
    }
}

pub fn self_anti_affinity<T>(x: T) -> Option<T>
where
    T: With<Affinity> + Has<Name>,
{
    // If it has no name associated, this is a noop
    let name: Option<Name> = x.get();
    name.and_then(anti_affinity).map(|aff| x.with(aff))
}

pub fn anti_affinity<T: Has<Name>>(x: T) -> Option<Affinity> {
    x.get().map(|name: Name| Affinity {
        pod_anti_affinity: Some(core::PodAntiAffinity {
            required_during_scheduling_ignored_during_execution: Some(vec![
                core::PodAffinityTerm {
                    label_selector: Some(LabelSelector {
                        match_labels: Some(BTreeMap::from([(Name::key(), name.into())])),
                        ..Default::default()
                    }),
                    topology_key: K8S_HOSTNAME.to_string(),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        }),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use k8s_openapi::api::core::v1::PodTemplateSpec;

    #[test]
    fn pod_anti_affinity() {
        let def: PodTemplateSpec = Default::default();
        let n = Name::new("ingester".to_string());
        let pt = def.with(n.clone());
        let x = super::self_anti_affinity(pt);
        assert_eq!(Some(n), x.get());
    }
}
