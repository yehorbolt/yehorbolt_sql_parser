format:
	cargo fmt && cargo clippy

test:
	cargo test

docs:
	cargo doc --open

cli:
	cargo run -- -p ./examples/txtfile.txt
	cargo run -- -p ./examples/sqlfile.sql
	cargo run -- -h
	cargo run -- -V