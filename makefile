rund:
	@clear
	@echo "---- tests ----"
	@cargo test
	@echo "---- run ----"
	@RUST_LOG=trace cargo run

run:
	@clear
	@echo "---- tests ----"
	@cargo test
	@echo "---- run ----"
	@RUST_LOG=warn cargo run

test:
	@cargo test
