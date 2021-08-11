build-HelloWorldFunction:
	cargo build --release
	cp ./target/release/bootstrap $(ARTIFACTS_DIR)
        aws sts get-caller-identity
