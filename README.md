# FeatureProbe Client Side SDK for Mobile Apps

Feature Probe is an open source feature management service. This SDK is used to control features in mobile programs.

## Basic Terms

Reading the short [Basic Terms](https://github.com/FeatureProbe/FeatureProbe/blob/main/BASIC_TERMS.md) will help to understand the code blow more easily.  [中文](https://github.com/FeatureProbe/FeatureProbe/blob/main/BASIC_TERMS_CN.md)

## Getting started

In this guide we explain how to use feature toggles in an application using FeatureProbe.

## Android Usage

### Kotlin

#### Step 1. Install SDK

```
implementation 'com.featureprobe:client-sdk-android:1.1.0@aar'
implementation "net.java.dev.jna:jna:5.7.0@aar"
```

#### Step 2. Create a FeatureProbe instance

```kotlin
import com.featureprobe.mobile.*;

val url = FpUrlBuilder("https://featureprobe.io/server").build();
val userId = /* unique user id in your business logic */
val user = FpUser(userId)
user.with("name", "bob")
val config = FpConfig(url!!, "client-9d885a68ca2955dfb3a7c95435c0c4faad70b50d", 10u, true)
val fp = FeatureProbe(config, user)
```

#### Step 3.  Use the feature toggle

``` kotlin
val showFeature = fp.boolValue("header_skin", false)
if (showFeature) {
    // application code to show the feature
} else {
    // the code to run if the feature is off
}
```

#### Step 4. Unit Testing (Optional)

```kotlin
val fp_for_test = FeatureProbe.newForTest("{ \"toggle_1\": true }")
val is_true = fp_for_test.boolValue("toggle_1", false)
assert(is_true == true)
```

Find the Demo code in [example](https://github.com/FeatureProbe/client-sdk-mobile/tree/main/sdk-android/app)

## iOS Usage

### Swift

#### Step 1. Install SDK

Swift Package Manager:

    1. XCode -> File -> Add Packages -> input `https://github.com/FeatureProbe/client-sdk-ios.git`
    2. click `Add Package`

Cocoapods:

    1. add `pod 'FeatureProbe', :git => 'git@github.com:FeatureProbe/client-sdk-ios.git'` to Podfile
    2. `pod install` or `pod update`

#### Step 2. Create a FeatureProbe instance

```swift
import featureprobe

let url = FpUrlBuilder(remoteUrl: "https://featureprobe.io/server").build();
let userId = /* unique user id in your business logic */
let user = FpUser(key: userId)
user.with(key: "name", value: "bob")
let config = FpConfig(
    remoteUrl: url!,
    clientSdkKey: "client-9d885a68ca2955dfb3a7c95435c0c4faad70b50d",
    refreshInterval: 10,
    waitFirstResp: true
)
let fp = FeatureProbe(config: config, user: user)
```

#### Step 3. Use the feature toggle

```swift
let showFeature = fp.boolValue(key: "header_skin", defaultValue: false)

if showFeature {
    // application code to show the feature
} else {
    // the code to run if the feature is off
}
```

#### Step 4. Unit Testing (Optional)

```swift
let fp2 = FeatureProbe.newForTest(toggles: "{ \"toggle_1\": true }")
let is_true = fp2.boolValue(key: "toggle_1", defaultValue: false)
assert(is_true == true);
```

Find the Demo code in [example](https://github.com/FeatureProbe/client-sdk-mobile/tree/main/sdk-ios/demo-cocoapods)

### Objective-C

#### Step 1. Install SDK

Cocoapods

add `pod 'FeatureProbe', :git => 'git@github.com:FeatureProbe/client-sdk-ios.git'` to Podfile

`pod install` or `pod update`

#### Step 2. Create a FeatureProbe instance

```objective-c
#import "FeatureProbe-Swift.h"

NSString *urlStr = @"https://featureprobe.io/server";
FpUrl *url = [[[FpUrlBuilder alloc] initWithRemoteUrl: urlStr] build];
NSString *userId = /* unique user id in your business logic */
FpUser *user = [[FpUser alloc] initWithKey: userId];
[user withKey:@"name" value:@"bob"];
FpConfig *config = [[FpConfig alloc] initWithRemoteUrl: url
                                          clientSdkKey:@"client-9d885a68ca2955dfb3a7c95435c0c4faad70b50d"
                                       refreshInterval: 10
                                         waitFirstResp: true];
FeatureProbe *fp = [[FeatureProbe alloc] initWithConfig:config user:user];
```

#### Step 3. Use the feature toggle

```objective-c
bool showFeature = [fp boolValueWithKey: @"header_skin" defaultValue: false];
if (showFeature) {
    // application code to show the feature
} else {
    // the code to run if the feature is off
}
```

#### Step 4. Unit Testing (Optional)

```objective-c
#import "FeatureProbe-Swift.h"

NSString *s = @"{ \"ab_test\": \"green\"}";
FeatureProbe *fp = [[FeatureProbe alloc] initWithTestJson: s];
NSString *value = [fp stringValueWithKey:@"ab_test" defaultValue:@"red"];
NSLog(@"value is %@", value);
```

Find the Demo code in [example](https://github.com/FeatureProbe/client-sdk-mobile/tree/main/sdk-ios/demo-objc)

## Testing

```shell
cargo test
```

## Contributing

We are working on continue evolving FeatureProbe core, making it flexible and easier to use.
Development of FeatureProbe happens in the open on GitHub, and we are grateful to the
community for contributing bugfixes and improvements.

Please read [CONTRIBUTING](https://github.com/FeatureProbe/featureprobe/blob/master/CONTRIBUTING.md)
for details on our code of conduct, and the process for taking part in improving FeatureProbe.
