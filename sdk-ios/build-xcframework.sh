#!/usr/bin/env bash
#
# This script builds the Rust crate in its directory into a staticlib XCFramework for iOS.

BUILD_PROFILE="release"
FRAMEWORK_NAME="FeatureProbeFFI"

while [[ "$#" -gt 0 ]]; do case $1 in
  --build-profile) BUILD_PROFILE="$2"; shift;shift;;
  --framework-name) FRAMEWORK_NAME="$2"; shift;shift;;
  *) echo "Unknown parameter: $1"; exit 1;
esac; done

THIS_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
PARENT_DIR="$( dirname "$THIS_DIR" )"
REPO_ROOT="$( dirname "$THIS_DIR" )"

MANIFEST_PATH="$REPO_ROOT/rust-uniffi/Cargo.toml"
echo $MANIFEST_PATH
if [[ ! -f "$MANIFEST_PATH" ]]; then
  echo "Could not locate Cargo.toml relative to script"
  exit 1
fi

LIB_NAME="libfeatureprobe_ffi.a"

####
##
## 1) Build the rust code individually for each target architecture.
##
####

# Helper to run the cargo build command in a controlled environment.
# It's important that we don't let environment variables from the user's default
# desktop build environment leak into the iOS build, otherwise it might e.g.
# link against the desktop build of NSS.

CARGO="$HOME/.cargo/bin/cargo"
LIBS_DIR="$REPO_ROOT/libs" # not work

DEFAULT_RUSTFLAGS=""
BUILD_ARGS=(+nightly build -Z  build-std=std,panic_abort  --manifest-path "$MANIFEST_PATH" --lib)
case $BUILD_PROFILE in
  debug) ;;
  release)
    BUILD_ARGS=("${BUILD_ARGS[@]}" --release)
    # With debuginfo, the zipped artifact quickly baloons to many
    # hundred megabytes in size. Ideally we'd find a way to keep
    # the debug info but in a separate artifact.
    DEFAULT_RUSTFLAGS="-C debuginfo=0"
    ;;
  *) echo "Unknown build profile: $BUILD_PROFILE"; exit 1;
esac

echo $REPO_ROOT

cargo_build () {
  TARGET=$1
  case $TARGET in
    x86_64*)
      LIBS_DIR="$REPO_ROOT/libs/ios/x86_64";;
    aarch64*)
      LIBS_DIR="$REPO_ROOT/libs/ios/arm64";;
    *)
      echo "Unexpected target architecture: $TARGET" && exit 1;;
  esac
  env -i \
    PATH="${PATH}" \
    RUSTC_WRAPPER="${RUSTC_WRAPPER:-}" \
    RUST_LOG="${RUST_LOG:-}" \
    RUSTFLAGS="${RUSTFLAGS:-$DEFAULT_RUSTFLAGS}" \
    "$CARGO" "${BUILD_ARGS[@]}" --target "$TARGET"
}

set -euvx

# Intel iOS simulator
CFLAGS_x86_64_apple_ios="-target x86_64-apple-ios" \
  cargo_build x86_64-apple-ios

# Hardware iOS targets
cargo_build aarch64-apple-ios

# M1 iOS simulator.
CFLAGS_aarch64_apple_ios_sim="--target aarch64-apple-ios-sim" \
  cargo_build aarch64-apple-ios-sim

####
##
## 2) Stitch the individual builds together an XCFramework bundle.
##
####

TARGET_DIR="$REPO_ROOT/target"
XCFRAMEWORK_ROOT="$THIS_DIR/$FRAMEWORK_NAME.xcframework"

# Start from a clean slate.

rm -rf "$XCFRAMEWORK_ROOT"

# Build the directory structure right for an individual framework.
# Most of this doesn't change between architectures.

COMMON="$XCFRAMEWORK_ROOT/common/$FRAMEWORK_NAME.framework"
PACKAGE_DIR="$THIS_DIR/client-sdk-ios"

mkdir -p "$COMMON/Modules"
cp "$THIS_DIR/module.modulemap" "$COMMON/Modules/"
cp $THIS_DIR/ObjcFeatureProbe.swift $PACKAGE_DIR/Sources/FeatureProbe
cp $THIS_DIR/LICENSE $PACKAGE_DIR/LICENSE
cp $THIS_DIR/FeatureProbe.podspec $PACKAGE_DIR/FeatureProbe.podspec
cp $THIS_DIR/Package.swift $PACKAGE_DIR/Package.swift
cp $THIS_DIR/README.md $PACKAGE_DIR/README.md

mkdir -p "$COMMON/Headers"
# it would be neat if there was a single UniFFI command that would spit out
# all of the generated headers for all UniFFIed dependencies of a given crate.
# For now we generate the Swift bindings to get the headers as a side effect,
# then delete the generated Swift code. Bleh.
uniffi-bindgen generate "$REPO_ROOT/rust-uniffi/src/featureprobe.udl" -l swift -o "$COMMON/Headers"
mv -f "$COMMON"/Headers/*.swift $PACKAGE_DIR/Sources/FeatureProbe

# Flesh out the framework for each architecture based on the common files.
# It's a little fiddly, because we apparently need to put all the simulator targets
# together into a single fat binary, but keep the hardware target separate.
# (TODO: we should try harder to see if we can avoid using `lipo` here, eliminating it
# would make the overall system simpler to understand).

# iOS hardware
mkdir -p "$XCFRAMEWORK_ROOT/ios-arm64"
cp -r "$COMMON" "$XCFRAMEWORK_ROOT/ios-arm64/$FRAMEWORK_NAME.framework"
cp "$TARGET_DIR/aarch64-apple-ios/$BUILD_PROFILE/$LIB_NAME" "$XCFRAMEWORK_ROOT/ios-arm64/$FRAMEWORK_NAME.framework/$FRAMEWORK_NAME"

# iOS simulator, with both platforms as a fat binary for mysterious reasons
mkdir -p "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator"
cp -r "$COMMON" "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator/$FRAMEWORK_NAME.framework"
lipo -create \
  -output "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator/$FRAMEWORK_NAME.framework/$FRAMEWORK_NAME" \
  "$TARGET_DIR/aarch64-apple-ios-sim/$BUILD_PROFILE/$LIB_NAME" \
  "$TARGET_DIR/x86_64-apple-ios/$BUILD_PROFILE/$LIB_NAME"

# Set up the metadata for the XCFramework as a whole.

cp "$THIS_DIR/Info.plist" "$XCFRAMEWORK_ROOT/Info.plist"

rm -rf "$XCFRAMEWORK_ROOT/common"

rm -rf "$PACKAGE_DIR/$FRAMEWORK_NAME.xcframework"
mv -f "$XCFRAMEWORK_ROOT" "$PACKAGE_DIR"
