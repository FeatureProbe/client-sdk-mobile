build_date = `date +%Y%m%d%H%M`
commit = `git rev-parse HEAD`
version = `git rev-parse --short HEAD`

build:
	make build_android
	make build_ios
clean:
	cargo clean
build_android:
	rustup component add rust-src
	cargo install --version 0.21.0  uniffi_bindgen
	rustup target add armv7-linux-androideabi
	rustup target add aarch64-apple-darwin
	rustup target add i686-linux-android
	rustup target add x86_64-linux-android
	rustup target add aarch64-linux-android
	cd sdk-android && ./gradlew clean && ./gradlew build
build_ios:
	rustup component add rust-src
	cargo install --version 0.21.0  uniffi_bindgen
	rustup target add aarch64-apple-ios
	rustup target add aarch64-apple-ios-sim
	rustup target add x86_64-apple-ios
	cd sdk-ios && ./build-xcframework.sh
test:
	cargo test --verbose


