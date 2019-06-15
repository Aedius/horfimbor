[![Build Status](https://travis-ci.org/Aedius/lignee.svg?branch=master)](https://travis-ci.org/Aedius/lignee)

# lignee

## initialize

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