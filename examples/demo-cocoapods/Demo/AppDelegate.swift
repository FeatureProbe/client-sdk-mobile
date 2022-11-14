import UIKit
import FeatureProbe

@UIApplicationMain
class AppDelegate: UIResponder, UIApplicationDelegate {

    var window: UIWindow?

    func testToggle() {
        let urlStr = "https://featureprobe.io/server"
        // let urlStr = "http://server_ip:4007" // for local docker
        let user = FpUser()
        user.with(key: "city", value: "1")
        let url = FpUrlBuilder(remoteUrl: urlStr).build()
        let config = FpConfig(
            remoteUrl: url!,
            // this key just for demo, you should copy from project list
            clientSdkKey: "client-25614c7e03e9cb49c0e96357b797b1e47e7f2dff",
            refreshInterval: 10,
            startWait: 2
        )
        let fp = FeatureProbe(config: config, user: user)
        let toggleValue = fp.boolDetail(key: "campaign_allow_list", defaultValue: false)
        print("toogle value is \( toggleValue)")
        fp.close() // stop sync toggles and flush events
    }

    func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplicationLaunchOptionsKey: Any]?) -> Bool {
        // Override point for customization after application launch.
        self.testToggle()
        return true
    }

    func applicationWillResignActive(_ application: UIApplication) {
        // Sent when the application is about to move from active to inactive state. This can occur for certain types of temporary interruptions (such as an incoming phone call or SMS message) or when the user quits the application and it begins the transition to the background state.
        // Use this method to pause ongoing tasks, disable timers, and invalidate graphics rendering callbacks. Games should use this method to pause the game.
    }

    func applicationDidEnterBackground(_ application: UIApplication) {
        // Use this method to release shared resources, save user data, invalidate timers, and store enough application state information to restore your application to its current state in case it is terminated later.
        // If your application supports background execution, this method is called instead of applicationWillTerminate: when the user quits.
    }

    func applicationWillEnterForeground(_ application: UIApplication) {
        // Called as part of the transition from the background to the active state; here you can undo many of the changes made on entering the background.
    }

    func applicationDidBecomeActive(_ application: UIApplication) {
        // Restart any tasks that were paused (or not yet started) while the application was inactive. If the application was previously in the background, optionally refresh the user interface.
    }

    func applicationWillTerminate(_ application: UIApplication) {
        // Called when the application is about to terminate. Save data if appropriate. See also applicationDidEnterBackground:.
    }


}

