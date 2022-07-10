import featureprobe

let url = FpUrlBuilder(remoteUrl: "http://127.0.0.1").build();
let user = FpUser(key: "key123")
user.setAttr(key: "city", value: "1")
let config = FpConfig(
    remoteUrl: url!,
    clientSdkKey: "client-1b31633671aa8be967697091b72d23da6bf858a7",
    refreshInterval: 10,
    waitFirstResp: true
)
let fp = FeatureProbe(config: config, user: user)
let toggle = fp.stringDetail(key: "ab_test", defaultValue: "blue")
print("toogle value is \(toggle)")

let fp2 = FeatureProbe.newForTest(toggles: "{ \"toggle_1\": true }")
let is_true = fp2.boolValue(key: "toggle_1", defaultValue: false)
assert(is_true == true);