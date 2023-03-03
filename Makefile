clean:
	kubectl delete deployment auto-ml
	kubectl delete service auto-ml
	minikube ssh -- docker system prune -a
	minikube stop
	docker system prune -a