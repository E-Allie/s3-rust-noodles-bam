build-s3Bam:
#	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release
#	cp ./target/x86_64-unknown-linux-gnu/release/bootstrap $(ARTIFACTS_DIR)
	cp ./target/debug/bootstrap $(ARTIFACTS_DIR)
