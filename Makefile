gen:
	cargo run --bin gen

deploy:
	kubectl apply -f ./build/crd.yaml

run:
	cargo run --bin secret-generator-rs

test:
	kubectl apply -f ./build/secret-generator-samples.yaml
