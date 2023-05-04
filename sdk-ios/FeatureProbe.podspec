Pod::Spec.new do |s|
  s.name         = 'FeatureProbe'
  s.version      = '2.2.0'
  s.license      = { :type => 'MIT' }
  s.homepage     = 'https://github.com/FeatureProbe/FeatureProbe'
  s.authors      = { 'featureprobe' => 'developer@featureprobe.com' }
  s.summary      = 'iOS feature probe SDK'
  s.source       = { :git => 'https://github.com/FeatureProbe/client-sdk-ios.git', :tag => s.version }
  s.source_files = 'Sources/**/*.swift'

  s.platform         = :ios, '10.0'
  s.pod_target_xcconfig = { 'VALID_ARCHS' => 'x86_64 arm64' }
  s.swift_versions = ['4.0', '4.2', '5.0', '5.5']
  s.vendored_frameworks = "FeatureProbeFFI.xcframework"
  s.pod_target_xcconfig = { 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386' }
  s.user_target_xcconfig = { 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386' }
end
