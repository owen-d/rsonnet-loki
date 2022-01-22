## TODO
env vars

## Concepts

* Exchange formats like YAML, JSON, etc are for _machines_, not humans. They are clunky, difficult to organize/parse/edit.
* Configuration languages are an anti-pattern. They lack the expressiveness and integrations (lsps) of programming languages.
  * Rather than try to make _another_ configuration language, use a programming language with good compiler guarantees, types, and editor integrations.
* Derive behavior to minimize misconfiguration opportunities.

## Patterns

Specification is _cheap_; don't bother with mutability. Much of this library is built on top
of the `Has<T>` and `With<T>` traits.

In general, use `Has<T>` when you want an `Option<T>` and `From<T>` when you want `<T>`.
I probably should have named that `Might<T>`...


### Modules

#### builtin

These are k8s primitives

#### paras

There are concepts built on top of the primitives. They can be used to overlay the primitives and give an idiomatic approach to some behavior or operating pattern. Examples inlcude:
* Using the `name` label in metadata for standardized selection like `name=<component>`
  * This same concept is used to name types (sts, deployments, svcs, etc)
  * We can derive compatible selectors (pvc mounts, svc targets, sts pod attachments, etc)
* Hashing the configmap(s) a pod mounts and including that in the pod's annotations to force rollouts when the configmap changes
* Building self-anti affinity rules so data nodes aren't scheduled together.
* Mounting volumes into consistently named paths within a container.

#### lokirs

This is an example usage of the library which creates a Loki deployment

## Notes
* Is it preferrable to use conventions to wrap structures externally? We could `with_config_hash` any `T: Has<ConfigMap> + Has<Name> + With<ConfigMap>`, etc. Then, we'd only need to specify this extra code (or remember to specify it at all!) _once_ at the end.
* Good demo
  * `Derive(Has<T>)` when a field matches
  * PrimitiveStream rewrite, i.e. config hash all `Has<Vec<ConfigMap>>` that are output by the program without ever having to specify it.
    * Show how to define this behavior and test for it
  * PrimitiveStream validation, i.e. ensure the combined requests or limits for a pod exceed the node resources for a particular machine class.
    * show how to define this behavior and test for it.
  * Load a manifest dump and apply validations.
* idea: should we get rid of `Has<T>` and use `From` and `TryFrom` instead?
