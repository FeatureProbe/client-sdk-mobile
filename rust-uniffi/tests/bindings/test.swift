import featureprobe

let url = FpUrlBuilder(remoteUrl: "https://featureprobe.io/server").build();
let user = FpUser()
user.with(key: "city", value: "1")
let config = FpConfig(
    remoteUrl: url!,
    clientSdkKey: "client-1b31633671aa8be967697091b72d23da6bf858a7",
    refreshInterval: 10,
    startWait: 5
)
let fp = FeatureProbe(config: config, user: user)
let toggle = fp.boolDetail(key: "campaign_enable", defaultValue: true)
print("toogle value is \(toggle)")
fp.close()

let fp2 = FeatureProbe.newForTest(toggles: "{ \"toggle_1\": true }")
let is_true = fp2.boolValue(key: "toggle_1", defaultValue: false)
assert(is_true == true);
