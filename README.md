## TODO
env vars

## Concepts

### builtin

These are k8s primitives

### paras

There are concepts built on top of the primitives. They can be used to overlay the primitives and give an idiomatic approach to some behavior or operating pattern. Examples inlcude:
* Using the `name` label in metadata for standardized selection lik `name=<component>`
* Hashing the configmap(s) a pod mounts and including that in the pod's annotations to force rollouts when the configmap changes
* Building self-anti affinity rules so data nodes aren't scheduled together.

### lokirs

This is an example usage of the library which creates a Loki deployment
