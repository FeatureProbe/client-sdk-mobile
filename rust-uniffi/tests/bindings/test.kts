import com.featureprobe.mobile.*;

val url = FpUrlBuilder("http://127.0.0.1").build();
val user = FpUser("123")
user.setAttr("city", "1")
val config = FpConfig(url!!, "client-1b31633671aa8be967697091b72d23da6bf858a7", 10u, true)
val fp = FeatureProbe(config, user)

val toggle = fp.stringDetail("ab_test", "blue")
println("toggle value is $toggle")