generate: src/* Cargo.toml
	cargo run --release > image.ppm

flamegraph: src/* Cargo.toml
	cargo flamegraph -F 50000 > image.ppm
