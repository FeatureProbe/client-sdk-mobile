import Foundation

@objc(FpConfig)
public final class OFpConfig: NSObject {
    var config: FpConfig
    
    @objc public init(remoteUrl: OFpUrl, clientSdkKey: String, refreshInterval: UInt8, waitFirstResp: Bool)  {
        let remoteUrl = remoteUrl._url
        config = FpConfig(remoteUrl: remoteUrl, clientSdkKey: clientSdkKey, refreshInterval: refreshInterval, waitFirstResp: waitFirstResp)
    }
    
}

@objc(FeatureProbe)
public final class OcFeatureProbe: NSObject {
    var fp: FeatureProbe
    
    @objc public init(config: OFpConfig, user: OFpUser)  {
        let config = config.config
        let user = user.user
        fp = FeatureProbe(config: config, user: user)
    }
    
    @objc public func boolValue(key: String, defaultValue: Bool)  -> Bool {
        fp.boolValue(key: key, defaultValue: defaultValue)
    }
    
    @objc public func boolDetail(key: String, defaultValue: Bool)  -> OFpBoolDetail {
        let d = fp.boolDetail(key: key, defaultValue: defaultValue)
        return OFpBoolDetail(detail: d)
    }
    
    @objc public func numberValue(key: String, defaultValue: Double)  -> Double {
        fp.numberValue(key: key, defaultValue: defaultValue)
    }
    
    @objc public func numberDetail(key: String, defaultValue: Double)  -> OFpNumberDetail {
        let d = fp.numberDetail(key: key, defaultValue: defaultValue)
        return OFpNumberDetail(detail: d)
    }
    
    @objc public func stringValue(key: String, defaultValue: String)  -> String {
        fp.stringValue(key: key, defaultValue: defaultValue)
    }
    
    @objc public func stringDetail(key: String, defaultValue: String)  -> OFpStringDetail {
        let d = fp.stringDetail(key: key, defaultValue: defaultValue)
        return OFpStringDetail(detail: d)
    }
    
    @objc public func jsonValue(key: String, defaultValue: String)  -> String {
        fp.jsonValue(key: key, defaultValue: defaultValue)
    }
    
    @objc public func jsonDetail(key: String, defaultValue: String)  -> OFpJsonDetail {
        let d = fp.jsonDetail(key: key, defaultValue: defaultValue)
        return OFpJsonDetail(detail: d)
    }
    
}

@objc(FpUser)
public final class OFpUser: NSObject {
    var user: FpUser
    
    @objc public init(key: String)  {
        let u = FpUser(key: key)
        user = u
    }
    
    @objc public func setAttr(key: String, value: String)  {
        user.setAttr(key: key, value: value)
    }
}

@objc(FpUrl)
public final class OFpUrl: NSObject {
    var _url: FpUrl
    
    public init(url: FpUrl)  {
        _url = url
    }
}

@objc(FpUrlBuilder)
public final class OFpUrlBuilder: NSObject {
    var builder: FpUrlBuilder
    
    @objc public init(remoteUrl: String)  {
        builder = FpUrlBuilder(remoteUrl: remoteUrl)
    }
    
    @objc public func build() -> OFpUrl? {
        let url = builder.build();
        if url == nil {
            return  nil
        }
        return OFpUrl(url: url!)
    }
    
}

@objc(FpBoolDetail)
public final class OFpBoolDetail: NSObject {
    var _detail: FpBoolDetail
    
    public init(detail: FpBoolDetail) {
        _detail = detail
    }
    
    @objc public var value: Bool {
        _detail.value
    }
    
    @objc public var ruleIndex: NSNumber {
        if _detail.ruleIndex == nil {
            return -1
        } else {
            return _detail.ruleIndex! as NSNumber
        }
    }
    
    @objc public var version: NSNumber {
        if _detail.version == nil {
            return -1
        } else {
            return _detail.version! as NSNumber
        }
    }
    
    @objc public var reason: String {
        _detail.reason
    }
}

@objc(FpNumberDetail)
public final class OFpNumberDetail: NSObject {
    var _detail: FpNumDetail
    
    public init(detail: FpNumDetail) {
        _detail = detail
    }
    
    @objc public var value: Double {
        _detail.value
    }
    
    @objc public var ruleIndex: NSNumber {
        if _detail.ruleIndex == nil {
            return -1
        } else {
            return _detail.ruleIndex! as NSNumber
        }
    }
    
    @objc public var version: NSNumber {
        if _detail.version == nil {
            return -1
        } else {
            return _detail.version! as NSNumber
        }
    }
    
    @objc public var reason: String {
        _detail.reason
    }
}

@objc(FpStringDetail)
public final class OFpStringDetail: NSObject {
    var _detail: FpStrDetail
    
    public init(detail: FpStrDetail) {
        _detail = detail
    }
    
    @objc public var value: String {
        _detail.value
    }
    
    @objc public var ruleIndex: NSNumber {
        if _detail.ruleIndex == nil {
            return -1
        } else {
            return _detail.ruleIndex! as NSNumber
        }
    }
    
    @objc public var version: NSNumber {
        if _detail.version == nil {
            return -1
        } else {
            return _detail.version! as NSNumber
        }
    }

    @objc public var reason: String {
        _detail.reason
    }
}

@objc(FpJsonDetail)
public final class OFpJsonDetail: NSObject {
    var _detail: FpJsonDetail
    
    public init(detail: FpJsonDetail) {
        _detail = detail
    }
    
    @objc public var value: String {
        _detail.value
    }
    
    @objc public var ruleIndex: NSNumber {
        if _detail.ruleIndex == nil {
            return -1
        } else {
            return _detail.ruleIndex! as NSNumber
        }
    }
    
    @objc public var version: NSNumber {
        if _detail.version == nil {
            return -1
        } else {
            return _detail.version! as NSNumber
        }
    }

    @objc public var reason: String {
        _detail.reason
    }
}
