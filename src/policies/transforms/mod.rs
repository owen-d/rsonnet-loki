// use k8s_openapi::api::core::v1::{PodSpec, PodTemplateSpec, Volume};

// use crate::paras::conventions::{with::WithMap, Has};

//
// pub fn hashed_configs<A: WithMap<PodTemplateSpec>>(x: A) -> A {
//     let tpl: PodTemplateSpec = x.has();
//     x.with_map(&|spec: PodTemplateSpec| PodTemplateSpec {
//         spec: spec.spec.map(|pod: PodSpec| {
//             let cfgs = pod
//                 .volumes
//                 .map(|vs| vs.iter().filter(|v| v.config_map.is_some()).collect());
//         }),
//         ..spec
//     })
// }

// use names in svc|metadata|sts|deployments
// anti affinity rules for sts|deploy
