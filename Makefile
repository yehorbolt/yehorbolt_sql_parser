format:
	cargo fmt && cargo clippy

PUBLISH_MESSAGE := "update"

publish:
	git add . && git commit -m $(PUBLISH_MESSAGE) && git push origin && cargo publish

test:
	cargo test

docs:
	cargo doc --no-deps --open

cli:
	cargo run -- -p ./examples/txtfile.txt
	cargo run -- -p ./examples/sqlfile.sql
	cargo run -- -h
	cargo run -- -V