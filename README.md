[![Build Status](https://travis-ci.org/Aedius/horfimbor.svg?branch=master)](https://travis-ci.org/Aedius/horfimbor)

# horfimbor

This project is licensed under the terms of the MIT license.

## initialize

### kafka

with a kubernetes server and helm installed :

    helm repo add incubator http://storage.googleapis.com/kubernetes-charts-incubator
    skaffold run -f.skaffold/kafka.yaml

to removed :

	skaffold delete -f.skaffold/kafka.yaml


### rust

For dev environnment

you need : 
- a [kubernetes](https://kubernetes.io/fr/) in local
- [skaffold](https://skaffold.dev/)

and then launch : 
> skaffold dev --port-forward

## add a sub project

- create a new rust project in a new folder
- create a k8s config
- add this config to skaffold.yaml
- add the folder in .travis.yml
- add the folder in dependabot


## extract Cargo.lock

in each sub project :

`docker build --target=build --tag=tmp . `
 
`docker run --rm --entrypoint cat tmp:latest  /horfimbor/Cargo.lock > Cargo.lock`