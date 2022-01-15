use k8s_openapi::api::core::v1::{Volume, VolumeMount};

use crate::builtin::{Has, VolumeMounts, Volumes, With};

/// Specify a volume should be mounted into the following container.
fn mount_with<F, A, B>(f: F, vs: A, x: B) -> B
where
    F: Fn(Volume) -> VolumeMount,
    A: Has<Volumes>,
    B: With<VolumeMounts>,
{
    x.with(vs.get().unwrap_or_default().into_iter().map(f).collect())
}

pub fn mount<A: Has<Volumes>, B: With<VolumeMounts>>(vs: A, x: B) -> B {
    mount_with(map_name, vs, x)
}

/// map_name maps a volume into a volumemount for use with
/// i.e. a container. It mounts into `/etc/volumes/<name>`
pub fn map_name(v: Volume) -> VolumeMount {
    VolumeMount {
        mount_path: format!("/etc/volumes/{}", v.name),
        name: v.name,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use k8s_openapi::api::core::v1::Container;

    use super::*;

    #[test]
    fn test_mount() {
        let c: Container = Default::default();
        let vs = vec![Volume {
            name: format!("foo"),
            ..Default::default()
        }];

        let kvs: Option<Vec<(String, String)>> = mount(vs, c)
            .volume_mounts
            .map(|ms| ms.into_iter().map(|m| (m.mount_path, m.name)).collect());

        assert_eq!(
            vec![(format!("/etc/volumes/foo"), format!("foo"))],
            kvs.unwrap(),
        )
    }
}