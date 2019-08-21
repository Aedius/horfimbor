

# start horfimbor
services-start: add-namespace
	skaffold dev -f.skaffold/horfimbor.yaml --port-forward

services-stop:
	skaffold delete -f.skaffold/services.yaml

add-namespace:
	kubectl create namespace horfimbor

