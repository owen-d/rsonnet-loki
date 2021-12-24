use k8s_openapi::api::apps::v1 as apps;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

pub fn idk() {
    let replicas = Some(12);
    let _dep_spec = apps::DeploymentSpec {
        replicas,
        strategy: Some(apps::DeploymentStrategy {
            rolling_update: Some(apps::RollingUpdateDeployment {
                max_unavailable: Some(IntOrString::String("10%".to_string())),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
}
