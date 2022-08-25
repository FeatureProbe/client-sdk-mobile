# FeatureProbe Android SDK

## Usage

1. add following to build.gradle

```console
implementation 'com.featureprobe:client-sdk-android:1.1.0@aar'
implementation "net.java.dev.jna:jna:5.7.0@aar"
```

2. add following to kotlin code

```kotlin
import com.featureprobe.mobile.*;

val url = FpUrlBuilder("https://featureprobe.io/server").build();
val user = FpUser()
user.with("city", "1")
val config = FpConfig(url!!, "client-9d885a68ca2955dfb3a7c95435c0c4faad70b50d", 10u, true)
val fp = FeatureProbe(config, user)

val toggleValue = fp.stringValue("ab_test", "red")
println("toggle value is $toggleValue")

```


## Build

0. make sure NDK version 21.3.6528147 is installed, and JNA jna.jar in $CLASSPATH

1. install uniffi codegen tool

`cargo install uniffi_bindgen`

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

## Unit Testing

```kotlin
val fp_for_test = FeatureProbe.newForTest("{ \"toggle_1\": true }")
val is_true = fp_for_test.boolValue("toggle_1", false)
assert(is_true == true)
```