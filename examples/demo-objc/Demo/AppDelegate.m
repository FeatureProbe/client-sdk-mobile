//
//  AppDelegate.m
//  Demo
//
//  Created by sebo on 2022/5/13.
//

#import "AppDelegate.h"
#import "FeatureProbe-Swift.h"

@interface AppDelegate ()

@end

@implementation AppDelegate


- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
    NSString *urlStr = @"https://featureprobe.io/server";
    // NSString *urlStr = @"http://server_ip:4007"; // for local docker
    
    FpUrl *url = [[[FpUrlBuilder alloc] initWithRemoteUrl: urlStr] build];
    FpUser *user = [[FpUser alloc] init];
    [user withKey:@"city" value:@"1"];
    
    // this key just for demo, you should copy from project list
    NSString *key = @"client-25614c7e03e9cb49c0e96357b797b1e47e7f2dff";
    FpConfig *config = [[FpConfig alloc] initWithRemoteUrl: url
                                              clientSdkKey: key
                                           refreshInterval: 10
                                             startWait: 2];
    
    FeatureProbe *fp = [[FeatureProbe alloc] initWithConfig:config user:user];
    FpBoolDetail *detail = [fp boolDetailWithKey:@"campaign_allow_list" defaultValue: false];
    NSLog(@"value is %d, reason is %@", detail.value, detail.reason);

    [fp close]; // stop sync toggles and flush events

    return YES;
}


#pragma mark - UISceneSession lifecycle


- (UISceneConfiguration *)application:(UIApplication *)application configurationForConnectingSceneSession:(UISceneSession *)connectingSceneSession options:(UISceneConnectionOptions *)options {
    // Called when a new scene session is being created.
    // Use this method to select a configuration to create the new scene with.
    return [[UISceneConfiguration alloc] initWithName:@"Default Configuration" sessionRole:connectingSceneSession.role];
}


- (void)application:(UIApplication *)application didDiscardSceneSessions:(NSSet<UISceneSession *> *)sceneSessions {
    // Called when the user discards a scene session.
    // If any sessions were discarded while the application was not running, this will be called shortly after application:didFinishLaunchingWithOptions.
    // Use this method to release any resources that were specific to the discarded scenes, as they will not return.
}


@end
