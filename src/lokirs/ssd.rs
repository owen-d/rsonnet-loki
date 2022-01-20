use k8s_openapi::api::apps::v1::{
    Deployment, DeploymentSpec, DeploymentStrategy, RollingUpdateDeployment,
};
use k8s_openapi::api::core::v1::{ConfigMap, PodTemplateSpec, Service};
use k8s_openapi::api::core::v1::{Container, PodSpec};


use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

use crate::builtin::configmap::with_config_hash;
use crate::builtin::{Name, With};
use crate::paras::affinity::anti_affinity;
use crate::paras::args::Target;
use crate::paras::mount::{self, mount_path};
use crate::paras::svc::cluster_ip;

pub const READ_NAME: &str = "read";
pub const WRITE_NAME: &str = "write";

pub struct SSD {
    pub image: String,
    pub read_replicas: i32,
    pub write_replicas: i32,
}

impl Default for SSD {
    fn default() -> Self {
        Self {
            image: "grafana/loki:main".to_string(),
            read_replicas: 3,
            write_replicas: 3,
        }
    }
}

impl SSD {
    fn read_name() -> Name {
        Name::new(READ_NAME.to_string())
    }
    fn write_name() -> Name {
        Name::new(WRITE_NAME.to_string())
    }
    pub fn read_deployment(&self) -> Deployment {
        let pod_template = with_config_hash(
            vec![super::config::config()],
            PodTemplateSpec {
                metadata: Some(Self::read_name().into()),
                spec: self
                    .pod_spec(
                        self.container().with(Target::new(WRITE_NAME.to_string())), // Add write target
                    )
                    .into(),
            },
        );
        let spec = DeploymentSpec {
            replicas: Some(self.read_replicas),
            selector: Self::read_name().into(),
            strategy: Some(DeploymentStrategy {
                rolling_update: Some(RollingUpdateDeployment {
                    max_unavailable: Some(IntOrString::String("10%".to_string())),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            template: pod_template,
            ..Default::default()
        };
        Deployment {
            metadata: Self::read_name().into(),
            spec: spec.into(),
            ..Default::default()
        }
    }
    pub fn config_map(&self) -> ConfigMap {
        super::config::config().into()
    }

    fn container(&self) -> Container {
        let cfg = super::config::config();
        Container {
            command: Some(vec![format!(
                "-config.file={}",
                mount_path(&cfg.clone().into())
            )]),
            image: Some(self.image.clone()),
            name: Self::read_name().into(),
            volume_mounts: Some(vec![mount::map_name(cfg.into())]),
            ..Default::default()
        }
    }

    fn pod_spec(&self, container: Container) -> PodSpec {
        let cfg = super::config::config();
        PodSpec {
            affinity: anti_affinity(Self::read_name()),
            containers: vec![container],
            volumes: Some(vec![cfg.into()]),
            ..Default::default()
        }
    }

    pub fn read_svc(&self) -> Service {
        cluster_ip(Self::read_name())
    }

    pub fn write_svc(&self) -> Service {
        cluster_ip(Self::write_name())
    }
}
