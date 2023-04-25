# FeatureProbe iOS-SDK

## How to use this SDK

See [iOS](https://docs.featureprobe.io/how-to/Client-Side%20SDKs/ios-sdk/) SDK Doc for detail. [苹果](https://docs.featureprobe.io/zh-CN/how-to/Client-Side%20SDKs/ios-sdk/)

## How to build

build from repo: `git@github.com:FeatureProbe/client-sdk-mobile.git`

1. install uniffi codegen tool

`cargo install --version 0.21 uniffi_bindgen`

2. install rust android target

```console
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios
```

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
