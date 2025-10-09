Lumio Node Deployment
=========================

This directory provides Terraform modules for a typical Lumio Node deployment, which includes both a validator node and fullnode, as well as HAProxy so that it's easy to manage incoming traffic. 

These Terraform modules are cloud-specific, and generally consist of a few high-level components:
* Cloud network configuration
* An installation of that cloud's managed Kubernetes service
* [Helm](https://helm.sh/) releases into that kubernetes cluster

If you wish to deploy an Lumio Node from scratch, Terraform is an easy way to spin that up on a public cloud. Alternatively, you may install the Helm charts directly on pre-existing Kubernetes clusters. After either step, you may refer to the Lumio Node helm operational docs here: https://github.com/lumio-labs/lumio-core/blob/main/terraform/helm/lumio-node/README.md
