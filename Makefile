.PHONY: run-cubes
run-cubes:
	RUST_LOG=voxel=trace,cubes=trace cargo run --example cubes

.PHONY: run-selection
run-selection:
	RUST_LOG=voxel=trace,selection=trace cargo run --example selection

all: run-ecs