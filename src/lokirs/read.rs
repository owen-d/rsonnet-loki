use k8s_openapi::api::apps::v1::{self as apps, DeploymentSpec};
use k8s_openapi::api::core::v1::PodSpec;
use k8s_openapi::api::core::v1::PodTemplateSpec;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

use crate::builtin::{Name, Volumes};
use crate::paras::affinity::anti_affinity;

pub const READ_NAME: &str = "read";

pub struct Reads {
    pub image: String,
    pub replicas: i32,
}

impl Default for Reads {
    fn default() -> Self {
        Self {
            image: "grafana/loki:main".to_string(),
            replicas: 3,
        }
    }
}

impl Reads {
    fn name() -> Name {
        Name::new(READ_NAME.to_string())
    }
    pub fn deployment_spec(&self) -> DeploymentSpec {
        apps::DeploymentSpec {
            replicas: Some(self.replicas),
            selector: Reads::name().get(),
            strategy: Some(apps::DeploymentStrategy {
                rolling_update: Some(apps::RollingUpdateDeployment {
                    max_unavailable: Some(IntOrString::String("10%".to_string())),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            template: PodTemplateSpec {
                metadata: Some(Self::name().into()),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn pod_spec() -> PodSpec {
        PodSpec {
            affinity: anti_affinity(Reads::name()),
            // containers: todo!(),
            volumes: Some(vec![super::config::config().into()]),
            ..Default::default()
        }
    }

    fn volumes(&self) -> Volumes {
        todo!()
    }
}
