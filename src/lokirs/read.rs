use k8s_openapi::api::apps::v1::{self as apps, DeploymentSpec};
use k8s_openapi::api::core::v1::PodTemplateSpec;
use k8s_openapi::api::core::v1::{Container, PodSpec};

use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

use crate::builtin::configmap::{with_config_hash};
use crate::builtin::{Name};
use crate::paras::affinity::anti_affinity;
use crate::paras::mount::{self, mount_path};

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
        let pod_template = with_config_hash(
            PodTemplateSpec {
                metadata: Some(Self::name().into()),
                spec: self.pod_spec().into(),
            },
            vec![super::config::config().into()],
        );
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
            template: pod_template,
            ..Default::default()
        }
    }

    // pub fn config_map(&self) -> HashableConfigMap {}

    fn pod_spec(&self) -> PodSpec {
        let cfg = super::config::config();
        PodSpec {
            affinity: anti_affinity(Reads::name()),
            containers: vec![Container {
                command: Some(vec![
                    "--config.file".to_string(),
                    mount_path(&cfg.clone().into()),
                    "--target".to_string(),
                    "read".to_string(),
                ]),
                image: Some(self.image.clone()),
                name: Self::name().into(),
                volume_mounts: Some(vec![mount::map_name(cfg.clone().into())]),
                ..Default::default()
            }],
            volumes: Some(vec![cfg.into()]),
            ..Default::default()
        }
    }
}
