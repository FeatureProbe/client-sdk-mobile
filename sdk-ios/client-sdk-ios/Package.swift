// swift-tools-version: 5.4
import PackageDescription

let package = Package(
    name: "FeatureProbe",
    platforms: [.iOS(.v9)],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "FeatureProbe",
            targets: ["FeatureProbe"]),
    ],
    dependencies: [
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .binaryTarget(
            name: "FeatureProbeFFI",
            path: "./FeatureProbeFFI.xcframework"
        ),
        .target(
            name: "FeatureProbe",
            dependencies: ["FeatureProbeFFI"]
        )
    ]
)
