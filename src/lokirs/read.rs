use k8s_openapi::api::apps::v1 as apps;
use k8s_openapi::api::core::v1::{PodSpec, PodTemplateSpec};
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

use crate::builtin::Name;
use crate::paras::affinity::anti_affinity;

pub const READ_NAME: &str = "read";

pub fn reads(replicas: i32) {
    let name = Name::new(READ_NAME.to_string());

    let _dep_spec = apps::DeploymentSpec {
        replicas: Some(replicas),
        selector: Name::new(READ_NAME.to_string()).get(),
        strategy: Some(apps::DeploymentStrategy {
            rolling_update: Some(apps::RollingUpdateDeployment {
                max_unavailable: Some(IntOrString::String("10%".to_string())),
                ..Default::default()
            }),
            ..Default::default()
        }),
        template: PodTemplateSpec {
            metadata: Some(name.into()),
            ..Default::default()
        },
        ..Default::default()
    };
}

pub fn read_pod_spec() -> PodSpec {
    let name = Name::new(READ_NAME.to_string());

    PodSpec {
        affinity: anti_affinity(name),
        containers: todo!(),
        volumes: todo!(),

        ..Default::default()
    }
}
