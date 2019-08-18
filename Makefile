

# start kafka
kafka-start: add-incubator
	skaffold run -f.skaffold/kafka.yaml

kafka-stop:
	skaffold delete -f.skaffold/kafka.yaml

add-incubator:
	helm repo add incubator http://storage.googleapis.com/kubernetes-charts-incubator
	helm repo update

# start lignee
services-start: add-namespace
	skaffold dev -f.skaffold/lignee.yaml --port-forward

services-stop:
	skaffold delete -f.skaffold/services.yaml

add-namespace:
	kubectl create namespace lignee

