package com.featureprobe.demo

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import com.featureprobe.mobile.*;
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        GlobalScope.launch(context = Dispatchers.IO) {
            val url = FpUrlBuilder("https://featureprobe.io/server").build();
            // val url = FpUrlBuilder("http://server_ip:4007").build(); // for local docker
            val user = FpUser()
            user.with("city", "1")
            val config = FpConfig(url!!, "client-1b31633671aa8be967697091b72d23da6bf858a7", 10u, true)
            val fp = FeatureProbe(config, user)
            while (true) {
                val toggleValue = fp.boolDetail("campaign_enable", false)
                Log.d("demo", "toggle value is $toggleValue")
                delay(3000)
            }
        }
    }
}