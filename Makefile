.PHONY: run-ecs
run-ecs:
	RUST_LOG=voxel=trace,ecs=trace cargo run --example ecs

all: run-ecs