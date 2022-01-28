pub const K8S_HOSTNAME: &str = "kubernetes.io/hostname";

// pub fn anti_affinity<T: Has<Name>>(x: T) -> Option<Affinity> {
//     x.get().map(|name: Name| Affinity {
//         pod_anti_affinity: Some(core::PodAntiAffinity {
//             required_during_scheduling_ignored_during_execution: Some(vec![
//                 core::PodAffinityTerm {
//                     label_selector: Some(LabelSelector {
//                         match_labels: Some(BTreeMap::from([(Name::key(), name.into())])),
//                         ..Default::default()
//                     }),
//                     topology_key: K8S_HOSTNAME.to_string(),
//                     ..Default::default()
//                 },
//             ]),
//             ..Default::default()
//         }),
//         ..Default::default()
//     })
// }
