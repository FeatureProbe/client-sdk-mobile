# FeatureProbe Android SDK

## How to use this SDK

See [Android](https://docs.featureprobe.io/sdk/Client-Side%20SDKs/android-sdk) SDK Doc for detail. [安卓](https://docs.featureprobe.io/zh-CN/sdk/Client-Side%20SDKs/android-sdk/)

## How to build

0. make sure NDK version 21.3.6528147 is installed, and JNA jna.jar in $CLASSPATH

1. install uniffi codegen tool

`cargo install --version 0.21 uniffi_bindgen`

2. install rust android target

```console
rustup target add armv7-linux-androideabi   # for arm
rustup target add aarch64-apple-darwin      # for darwin arm64 (if you have a M1 MacOS)
rustup target add i686-linux-android        # for x86
rustup target add x86_64-linux-android
rustup target add aarch64-linux-android
```

3. build android lib

./gradlew build  
