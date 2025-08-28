generate: src/* Cargo.toml
	cargo run --release

flamegraph: src/* Cargo.toml
	cargo flamegraph -F 50000
