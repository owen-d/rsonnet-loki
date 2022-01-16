use k8s_openapi::api::core::v1::ConfigMap;
use maplit::btreemap;

use crate::builtin::Name;

pub const DEFAULT_CONFIG: &str = "
auth_enabled: false

server:
  http_listen_port: 3100
  grpc_listen_port: 9096

common:
  path_prefix: /tmp/loki
  storage:
    filesystem:
      chunks_directory: /tmp/loki/chunks
      rules_directory: /tmp/loki/rules
  replication_factor: 1
  ring:
    instance_addr: 127.0.0.1
    kvstore:
      store: inmemory

schema_config:
  configs:
    - from: 2021-01-01
      store: boltdb-shipper
      object_store: filesystem
      schema: v11
      index:
        prefix: index_
        period: 24h
";

pub fn config() -> ConfigMap {
    ConfigMap {
        metadata: Name::new("loki".to_string()).into(),
        data: Some(btreemap! {
            format!("config.yaml") => format!("{}", DEFAULT_CONFIG),
        }),
        ..Default::default()
    }
}
