# FeatureProbe iOS-SDK

## Usage

Currently support two kinds of package manager:

1. Swift Package Manager
    XCode -> File -> Add Packages -> input `https://github.com/FeatureProbe/client-sdk-ios.git`

2. Cocoapods
    add `pod 'FeatureProbe', :git => 'git@github.com:FeatureProbe/client-sdk-ios.git'` to Podfile
    `pod install` or `pod update`

```swift
import FeatureProbe

let url = FpUrlBuilder(remoteUrl: "remote_url").build();
let user = FpUser(key: "key123")
user.setAttr(key: "city", value: "1")
let config = FpConfig(
    remoteUrl: url!,
    clientSdkKey: "client-9d885a68ca2955dfb3a7c95435c0c4faad70b50d",
    refreshInterval: 10,
    waitFirstResp: true
)
let fp = FeatureProbe(config: config, user: user)
let toggleValue = fp.stringDetail(key: "ab_test", defaultValue: "red")
print("toogle value is \(toggleValue)")

```

```objective-c
#import "FeatureProbe-Swift.h"

NSString *urlStr = @"remote_url";
FpUrl *url = [[[FpUrlBuilder alloc] initWithRemoteUrl: urlStr] build];
FpUser *user = [[FpUser alloc] initWithKey:@"user_key"];
FpConfig *config = [[FpConfig alloc] initWithRemoteUrl: url clientSdkKey:@"client-9d885a68ca2955dfb3a7c95435c0c4faad70b50d" refreshInterval: 10 waitFirstResp: true];

FeatureProbe *fp = [[FeatureProbe alloc] initWithConfig:config user:user];
NSString *value = [fp stringValueWithKey:@"ab_test" defaultValue:@"red"];
NSLog(@"value is %@", value);

```

## Build
build from repo: `git@github.com:FeatureProbe/client-sdk-mobile.git`

1. install uniffi codegen tool

cargo install uniffi_bindgen

2. install rust android target

rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios

3. build xcframework

`./build-xcframework.sh`

4. push to git

```
cd client-sdk-ios
git commit -m 'xxx'
git push origin master
```


## Contributing
We are working on continue evolving FeatureProbe core, making it flexible and easier to use.
Development of FeatureProbe happens in the open on GitHub, and we are grateful to the
community for contributing bugfixes and improvements.

Please read [CONTRIBUTING](https://github.com/FeatureProbe/featureprobe/blob/master/CONTRIBUTING.md)
for details on our code of conduct, and the process for taking part in improving FeatureProbe.
