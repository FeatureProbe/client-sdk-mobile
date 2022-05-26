build_date = `date +%Y%m%d%H%M`
commit = `git rev-parse HEAD`
version = `git rev-parse --short HEAD`

.PHONY: release
clean:
	cargo clean
release:
	 (ssh-agent -k || true) && \
		eval `ssh-agent -s` && \
		ssh-add && \
		cargo build --release --verbose
release-test:
	 (ssh-agent -k || true) && \
		eval `ssh-agent -s` && \
		ssh-add && \
		cargo test --release --verbose
test:
	cargo test --verbose


