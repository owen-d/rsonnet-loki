use k8s_openapi::api::apps::v1::{
    Deployment, DeploymentSpec, DeploymentStrategy, RollingUpdateDeployment,
};
use k8s_openapi::api::core::v1::{ConfigMap, PodTemplateSpec, Service, ServicePort, ServiceSpec};
use k8s_openapi::api::core::v1::{Container, PodSpec};

use k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector;
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;

use crate::builtin::configmap::with_config_hash;
use crate::builtin::Name;
use crate::paras::affinity::anti_affinity;
use crate::paras::mount::{self, mount_path};

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
                spec: self.pod_spec().into(),
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

    fn pod_spec(&self) -> PodSpec {
        let cfg = super::config::config();
        PodSpec {
            affinity: anti_affinity(Self::read_name()),
            containers: vec![Container {
                command: Some(vec![
                    format!("-config.file={}", mount_path(&cfg.clone().into())),
                    "-target=read".to_string(),
                ]),
                image: Some(self.image.clone()),
                name: Self::read_name().into(),
                volume_mounts: Some(vec![mount::map_name(cfg.clone().into())]),
                ..Default::default()
            }],
            volumes: Some(vec![cfg.into()]),
            ..Default::default()
        }
    }

    pub fn read_svc(&self) -> Service {
        let sel: LabelSelector = Self::read_name().into();
        Service {
            metadata: Self::read_name().into(),
            spec: Some(ServiceSpec {
                cluster_ip: "".to_string().into(),
                ports: Some(vec![
                    ServicePort {
                        name: format!("{}-http", Self::read_name().0).into(),
                        port: 3100,
                        target_port: IntOrString::Int(3100).into(),
                        protocol: Some("tcp".to_string()),
                        ..Default::default()
                    },
                    ServicePort {
                        name: format!("{}-grpc", Self::read_name().0).into(),
                        port: 9095,
                        target_port: IntOrString::Int(9095).into(),
                        protocol: Some("tcp".to_string()),
                        ..Default::default()
                    },
                ]),
                selector: sel.match_labels,
                type_: Some("ClusterIP".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
