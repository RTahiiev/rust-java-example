.PHONY: run

run:
	chmod +x run.sh && ./run.sh

.PHONY: build

build:
	cargo build --release
