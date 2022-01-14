## TODO
env vars

## Concepts

* Exchange formats like YAML, JSON, etc are for _machines_, not humans. They are clunky, difficult to organize/parse/edit.
* Configuration languages are an anti-pattern. They lack the expressiveness and integrations (lsps) of programming languages.
  * Rather than try to make _another_ configuration language, use a programming language with good compiler guarantees, types, and editor integrations.
* Derive as much as reasonable to minimize misconfiguration opportunities.

## Patterns

Specification is _cheap_; don't mess with mutability we don't need. Much of this library is built on top
of the `Has<T>` and `With<T>` traits.


### Modules

#### builtin

These are k8s primitives

#### paras

There are concepts built on top of the primitives. They can be used to overlay the primitives and give an idiomatic approach to some behavior or operating pattern. Examples inlcude:
* Using the `name` label in metadata for standardized selection lik `name=<component>`
* Hashing the configmap(s) a pod mounts and including that in the pod's annotations to force rollouts when the configmap changes
* Building self-anti affinity rules so data nodes aren't scheduled together.
* Mounting volumes into consistently named paths within a container.

#### lokirs

This is an example usage of the library which creates a Loki deployment
