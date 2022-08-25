import com.featureprobe.mobile.*;

val url = FpUrlBuilder("https://featureprobe.io/server").build()
val user = FpUser("123")
user.with("city", "1")
val config = FpConfig(url!!, "client-1b31633671aa8be967697091b72d23da6bf858a7", 10u, true)
val fp = FeatureProbe(config, user)
fp.close()

val toggle = fp.boolDetail("header_skin", true)
println("toggle value is $toggle")

val fp_for_test = FeatureProbe.newForTest("{ \"toggle_1\": true }")
val is_true = fp_for_test.boolValue("toggle_1", false)
assert(is_true == true)
