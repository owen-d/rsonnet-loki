use super::conventions::Name;
use k8s_openapi::api::core::v1::VolumeMount;

pub fn mount_path(n: Name) -> String {
    format!("/etc/volumes/{}", n)
}

/// mount_name maps a volume into a volumemount for use with
/// i.e. a container. It mounts into `/etc/volumes/<name>`
pub fn mount_name(n: Name) -> VolumeMount {
    VolumeMount {
        mount_path: mount_path(n.clone()),
        name: n.into(),
        ..Default::default()
    }
}

impl From<VolumeMount> for Name {
    fn from(v: VolumeMount) -> Self {
        Self(v.name)
    }
}
