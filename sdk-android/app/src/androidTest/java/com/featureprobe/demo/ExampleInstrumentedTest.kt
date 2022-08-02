package com.featureprobe.demo

import android.util.Log
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.featureprobe.mobile.FpConfig
import com.featureprobe.mobile.newFeatureProbe
import com.featureprobe.mobile.newFpUser
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

import org.junit.Test
import org.junit.runner.RunWith

import org.junit.Assert.*

/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class ExampleInstrumentedTest {
    @Test
    fun useAppContext() {
        // Context of the app under test.
        val appContext = InstrumentationRegistry.getInstrumentation().targetContext
        assertEquals("com.featureprobe.androidsdk", appContext.packageName)

        val url =
            "remote_url/api/client-sdk/toggles";
        val user = newFpUser("123")
        user.with("city", "1")
        val config = FpConfig(url, "client-9d885a68ca2955dfb3a7c95435c0c4faad70b50d", 10u, true)
        val fp = newFeatureProbe(config, user)
        assert(fp != null)
        val toggleValue = fp?.strDetail("ab_test", "red")
        Log.d("demo", "toggle value is $toggleValue")
    }
}