gen:
	cargo run --bin gen

gen-preview:
	cargo run --bin gen-preview

deploy:
	kubectl apply -k ./manifests

undeploy:
	kubectl delete -k ./manifests

run:
	cargo run --bin secret-generator-rs

test:
	kubectl apply -f ./examples/secret-generator-samples.yaml

.PHONY: build
build:
	docker build -t locmai/secret-generator-rs:0.0.1 .

push:
	docker push locmai/secret-generator-rs:0.0.1

clean:
	rm -rf ./target
	kubectl delete -k ./manifests
