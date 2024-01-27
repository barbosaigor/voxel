.PHONY: run-ecs
run-ecs:
	RUST_LOG=voxel=trace,ecs=trace cargo run --example ecs

.PHONY: run-rapier
run-rapier:
	RUST_LOG=voxel=trace,ecs=trace cargo run --example rapier

all: run-ecs