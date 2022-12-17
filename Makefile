.PHONY: run-ecs
run-cubes:
	RUST_LOG=voxel=trace,ecs=trace cargo run --example cubes

all: run-ecs