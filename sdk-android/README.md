# FeatureProbe Android SDK

## Build

1. install uniffi codegen tool

cargo install uniffi_bindgen

2. install rust android target

rustup target add armv7-linux-androideabi   # for arm
rustup target add aarch64-apple-darwin      # for darwin arm64 (if you have a M1 MacOS)
rustup target add i686-linux-android        # for x86
rustup target add x86_64-linux-android
rustup target add aarch64-linux-android

3. build android lib

./gradlew build  

## Usage

1. add following to build.gradle

`implementation 'com.featureprobe.mobile:android_sdk:1.0.1'`
`implementation "net.java.dev.jna:jna:5.7.0@aar"`

2. add following to kotlin code

```kotlin
import com.featureprobe.mobile.*;

val url = FpUrlBuilder("remote_url/api/client-sdk/toggles").build();
val user = FpUser("123")
user.setAttr("city", "1")
val config = FpConfig(url!!, "client-9d885a68ca2955dfb3a7c95435c0c4faad70b50d", 10u, true)
val fp = FeatureProbe(config, user)

val toggleValue = fp.stringValue("ab_test", "red")
println("toggle value is $toggleValue")

```
