use k8s_openapi::api::apps::v1 as apps;
use k8s_openapi::api::core::v1 as core;
use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

pub fn new(name: String, replicas: i32, pod: core::PodTemplateSpec) -> apps::Deployment {
    let spec = apps::DeploymentSpec {
        replicas: Some(replicas),
        strategy: Some(apps::DeploymentStrategy {
            rolling_update: Some(apps::RollingUpdateDeployment {
                max_unavailable: Some(IntOrString::String("10%".to_string())),
                ..Default::default()
            }),
            ..Default::default()
        }),
        template: pod,
        ..Default::default()
    };
    apps::Deployment {
        metadata: meta::ObjectMeta {
            name: Some(name),
            ..Default::default()
        },
        spec: Some(spec),
        ..Default::default()
    }
}
