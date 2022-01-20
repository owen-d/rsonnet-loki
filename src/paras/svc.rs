use k8s_openapi::{
    api::core::v1::{Service, ServicePort, ServiceSpec},
    apimachinery::pkg::{apis::meta::v1::LabelSelector, util::intstr::IntOrString},
};

use crate::builtin::Name;

pub enum Port {
    Http { port: i32 },
    Grpc { port: i32 },
    Grpclb { port: i32 },
}

impl Port {
    pub fn http() -> Port {
        Port::Http { port: 3100 }
    }
    pub fn grpc() -> Port {
        Port::Grpc { port: 9095 }
    }
    pub fn grpclb() -> Port {
        Port::Grpclb { port: 9095 }
    }
}

impl From<Port> for ServicePort {
    fn from(p: Port) -> Self {
        match p {
            Port::Http { port } => ServicePort {
                name: String::from("http-metrics").into(),
                port,
                protocol: Some("tcp".to_string()),
                target_port: IntOrString::Int(port).into(),
                ..Default::default()
            },
            Port::Grpc { port } => ServicePort {
                name: String::from("grpc").into(),
                port,
                protocol: Some("tcp".to_string()),
                target_port: IntOrString::Int(port).into(),
                ..Default::default()
            },
            Port::Grpclb { port } => ServicePort {
                name: String::from("grpclb").into(),
                port,
                protocol: Some("tcp".to_string()),
                target_port: IntOrString::Int(port).into(),
                ..Default::default()
            },
        }
    }
}

/// cluster_ip svc type
pub fn cluster_ip(name: Name) -> Service {
    let sel: LabelSelector = name.clone().into();
    Service {
        metadata: name.into(),
        spec: Some(ServiceSpec {
            ports: Some(vec![Port::http().into(), Port::grpc().into()]),
            selector: sel.match_labels,
            type_: Some("ClusterIP".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

/// discovery svc type, uses ClusterIP=None and publishNotReadyAddresses=true
pub fn discovery(name: Name) -> Service {
    let sel: LabelSelector = name.clone().into();
    Service {
        metadata: name.into(),
        spec: Some(ServiceSpec {
            ports: Some(vec![Port::http().into(), Port::grpclb().into()]),
            selector: sel.match_labels,
            type_: Some("ClusterIP".to_string()),
            publish_not_ready_addresses: Some(true),
            cluster_ip: Some("None".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    }
}
