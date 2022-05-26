# FeatureProbe Client Side SDK for Mobile Apps

Feature Probe is an open source feature management service. This SDK is used to control features in mobile programs.

## Getting started

In this guide we explain how to use feature toggles in an application using FeatureProbe.

## Android Usage

### Step 1. Install SDK

`implementation 'com.featureprobe.mobile:android_sdk:1.0.1'`

`implementation "net.java.dev.jna:jna:5.7.0@aar"`

### Step 2. Create a FeatureProbe instance

```kotlin
import com.featureprobe.mobile.*;

val url = FpUrlBuilder("remote_url/").build();
val user = FpUser("user@company.com")
user.setAttr("name", "bob")
val config = FpConfig(url!!, "client-9d885a68ca2955dfb3a7c95435c0c4faad70b50d", 10u, true)
val fp = FeatureProbe(config, user)
```

### Step 3.  Use the feature toggle

``` kotlin
val showFeature = fp.boolValue("your.toggle.key", false)
if (showFeature) {
    # application code to show the feature
} else {
    # the code to run if the feature is off
}
```

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

let url = FpUrlBuilder(remoteUrl: "remote_url/").build();
let user = FpUser(key: "user@company.com")
user.setAttr(key: "name", value: "bob")
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
let showFeature = fp.boolValue("your.toggle.key", false)
if showFeature {
    # application code to show the feature
} else {
    # the code to run if the feature is off
}
```

### Objective-C

#### Step 1. Install SDK

Cocoapods

add `pod 'FeatureProbe', :git => 'git@github.com:FeatureProbe/client-sdk-ios.git'` to Podfile

`pod install` or `pod update`

#### Step 2. Create a FeatureProbe instance

```objective-c
#import "FeatureProbe-Swift.h"

NSString *urlStr = @"remote_url/";
FpUrl *url = [[[FpUrlBuilder alloc] initWithRemoteUrl: urlStr] build];
FpUser *user = [[FpUser alloc] initWithKey:@"user_key"];
[user setAttrWithKey:@"name" value:@"bob"];
FpConfig *config = [[FpConfig alloc] initWithRemoteUrl: url
                                          clientSdkKey:@"client-9d885a68ca2955dfb3a7c95435c0c4faad70b50d"
                                       refreshInterval: 10
                                         waitFirstResp: true];
FeatureProbe *fp = [[FeatureProbe alloc] initWithConfig:config user:user];
```

#### Step 3. Use the feature toggle

```objective-c
bool showFeature = [fp boolValueWithKey: @"your.toggle.key" defaultValue: false];
if (showFeature) {
    # application code to show the feature
} else {
    # the code to run if the feature is off
}
```

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
