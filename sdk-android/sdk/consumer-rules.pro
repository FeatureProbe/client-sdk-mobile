# feature probe sdk
-keep class com.featureprobe.mobile.* { *; }

# jna
-dontwarn java.awt.*
-keep class com.sun.jna.* { *; }
-keepclassmembers class * extends com.sun.jna.* { public *; }