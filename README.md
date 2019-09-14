[![Build Status](https://travis-ci.org/Aedius/horfimbor.svg?branch=master)](https://travis-ci.org/Aedius/horfimbor)

# horfimbor

This project is licensed under the terms of the MIT license.

## initialize

### install microk8s for dev from scratch ( on debian 9 )

	aptitude install snapd
	snap install core
	snap install microk8s --classic
	# relog
	microk8s.enable dashboard storage registry ingress dns
	# go to http://<your-ip>:16443/api/v1/namespaces/kube-system/services/https:kubernetes-dashboard:/proxy/#!/login

	# to get user / password :
	kubectl config view 
	
	# to get the token :
	token=$(microk8s.kubectl -n kube-system get secret | grep default-token | cut -d " " -f1)
	microk8s.kubectl -n kube-system describe secret $token
	
	# to copy the config into a file
	microk8s.kubectl config view --raw > $HOME/.kube/config 
	# to connect from an other server, use this file, replacing 127.0.0.1 by your ip

### kafka ( with zookeeper )

with a kubernetes server and helm installed :

    helm repo add incubator http://storage.googleapis.com/kubernetes-charts-incubator
    helm repo update
    kubectl apply -f ./.helm/namespace.yml
    helm upgrade --wait --install zookeeper --namespace infra incubator/zookeeper --values ./.helm/zookeeper/values.yml
    helm upgrade --wait --install kafka --namespace infra incubator/kafka --values ./.helm/kafka/values.yml

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
