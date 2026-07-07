import Foundation
import XCTest
@testable import Compiler

final class CppRustModuleBridgeGeneratorTests: XCTestCase {
    func testGeneratedConcreteModelHelpersAreEmitted() throws {
        let bundleInfo = try makeRustTestBundleInfo()
        let moduleCppType = makeCppType(name: "RustTestModule", bundleInfo: bundleInfo)
        let payloadCppType = makeCppType(name: "CounterPayload", bundleInfo: bundleInfo)
        let payloadModel = ValdiModel(tsType: "CounterPayload",
                                      iosType: nil,
                                      androidClassName: nil,
                                      cppType: payloadCppType,
                                      typeParameters: nil,
                                      exportAsInterface: false,
                                      legacyConstructors: false,
                                      usePublicFields: false,
                                      comments: nil,
                                      properties: [
                                        modelProperty(name: "label", type: .string),
                                        modelProperty(name: "value", type: .double),
                                      ])
        var payloadMapping = ValdiNodeClassMapping(tsType: "CounterPayload",
                                                   iosType: nil,
                                                   androidClassName: nil,
                                                   cppType: payloadCppType,
                                                   kind: .class)
        payloadMapping.isGenerated = true

        let payloadType = ValdiModelPropertyType.object(payloadMapping)
        let moduleModel = ValdiModel(tsType: "RustTestModule",
                                     iosType: nil,
                                     androidClassName: nil,
                                     cppType: moduleCppType,
                                     typeParameters: nil,
                                     exportAsInterface: true,
                                     legacyConstructors: false,
                                     usePublicFields: false,
                                     comments: nil,
                                     properties: [
                                        modelProperty(name: "echoPayload",
                                                      type: .function(parameters: [
                                                        modelProperty(name: "payload", type: payloadType),
                                                      ],
                                                      returnType: payloadType,
                                                      isSingleCall: false,
                                                      shouldCallOnWorkerThread: false,
                                                      allowSyncCall: false)),
                                     ])
        let exportedModule = ExportedModule(model: moduleModel, modulePath: "rust_test/RustCounter")
        let generator = CppRustModuleBridgeGenerator(bundleInfo: bundleInfo,
                                                     cppType: moduleCppType,
                                                     exportedModule: exportedModule,
                                                     sourceFileName: GeneratedSourceFilename(filename: "RustCounter.d.ts",
                                                                                             symbolName: "RustTestModule"),
                                                     generatedModelsByTypeName: [
                                                        "CounterPayload": payloadModel,
                                                        payloadCppType.declaration.fullTypeName: payloadModel,
                                                     ])

        let sources = try generator.write()
        let rustSource = try source(named: "rust_test.rust_bridge.rs", in: sources)
        let cppSource = try source(named: "RustTestModuleRustBridge.cpp", in: sources)

        XCTAssertTrue(rustSource.contains("pub struct CounterPayload(crate::valdi_rust::ValdiRustRetainedHandle<CounterPayloadHandleMarker>);"))
        XCTAssertTrue(rustSource.contains("pub fn new(label: String, value: Double) -> Self"))
        XCTAssertTrue(rustSource.contains("pub fn get_label(&self) -> String"))
        XCTAssertTrue(rustSource.contains("pub fn get_value(&self) -> Double"))
        XCTAssertTrue(rustSource.contains("valdi_rust_module::echo_payload(valdi_rust_module::CounterPayload::from_handle(payload)).into_return_handle()"))

        XCTAssertTrue(cppSource.contains("extern \"C\" ValdiRustHandle valdi_rust_rust_test_rust_test_module_counter_payload_new"))
        XCTAssertTrue(cppSource.contains("modelBox->value.getLabel()"))
        XCTAssertTrue(cppSource.contains("modelBox->value.getValue()"))
        XCTAssertTrue(cppSource.contains("modelBox->value.setLabel("))
    }

    func testUnsupportedNativeObjectWithoutConverterIsRejected() throws {
        let bundleInfo = try makeRustTestBundleInfo()
        let moduleCppType = makeCppType(name: "RustTestModule", bundleInfo: bundleInfo)
        let foreignCppType = makeCppType(name: "ForeignNative", bundleInfo: bundleInfo)
        let foreignMapping = ValdiNodeClassMapping(tsType: "ForeignNative",
                                                   iosType: nil,
                                                   androidClassName: nil,
                                                   cppType: foreignCppType,
                                                   kind: .class)
        let foreignType = ValdiModelPropertyType.object(foreignMapping)
        let moduleModel = ValdiModel(tsType: "RustTestModule",
                                     iosType: nil,
                                     androidClassName: nil,
                                     cppType: moduleCppType,
                                     typeParameters: nil,
                                     exportAsInterface: true,
                                     legacyConstructors: false,
                                     usePublicFields: false,
                                     comments: nil,
                                     properties: [
                                        modelProperty(name: "useForeign",
                                                      type: .function(parameters: [
                                                        modelProperty(name: "value", type: foreignType),
                                                      ],
                                                      returnType: .void,
                                                      isSingleCall: false,
                                                      shouldCallOnWorkerThread: false,
                                                      allowSyncCall: false)),
                                     ])
        let exportedModule = ExportedModule(model: moduleModel, modulePath: "rust_test/RustCounter")
        let generator = CppRustModuleBridgeGenerator(bundleInfo: bundleInfo,
                                                     cppType: moduleCppType,
                                                     exportedModule: exportedModule,
                                                     sourceFileName: GeneratedSourceFilename(filename: "RustCounter.d.ts",
                                                                                             symbolName: "RustTestModule"))

        XCTAssertThrowsError(try generator.write()) { error in
            XCTAssertTrue(error.legibleLocalizedDescription.contains(
                "Rust native modules do not support exported module function 'useForeign' function parameter 'value' type 'ForeignNative'"
            ))
        }
    }

    private func makeRustTestBundleInfo() throws -> CompilationItem.BundleInfo {
        let tempDir = URL(fileURLWithPath: NSTemporaryDirectory(), isDirectory: true)
            .appendingPathComponent(UUID().uuidString, isDirectory: true)
        try FileManager.default.createDirectory(at: tempDir, withIntermediateDirectories: true)
        let configURL = tempDir.appendingPathComponent("valdi.yml")
        try """
        base_dir: .
        project_name: RustBridgeTests
        compiler_toolbox_path: ./compiler-toolbox
        cpp:
          rust_bridge_enabled: true
        """.write(to: configURL, atomically: true, encoding: .utf8)

        let logger = Logger(output: BufferLoggerOutput())
        logger.asyncLogs = false
        let args = try ValdiCompilerArguments.parse(["--compile"])
        let projectConfig = try ValdiProjectConfig.from(logger: logger,
                                                        configUrl: configURL,
                                                        currentDirectoryUrl: tempDir,
                                                        environment: [:],
                                                        args: args)

        return try CompilationItem.BundleInfo(name: "rust_test",
                                              baseDir: tempDir,
                                              relativeProjectPathPrefix: "rust_test/",
                                              disableHotReload: true,
                                              disableAnnotationProcessing: false,
                                              disableDependencyVerification: false,
                                              disableBazelBuildFileGeneration: true,
                                              asyncStrictMode: false,
                                              webNpmScope: "",
                                              webVersion: "0.0.0",
                                              webPublishConfig: "",
                                              webMain: "",
                                              iosModuleName: "RustTest",
                                              iosLanguage: .objc,
                                              iosClassPrefix: nil,
                                              androidClassPath: nil,
                                              androidExportStrings: false,
                                              cppClassPrefix: nil,
                                              iosCodegenEnabled: false,
                                              androidCodegenEnabled: false,
                                              cppCodegenEnabled: true,
                                              disableCodeCoverage: true,
                                              singleFileCodegen: false,
                                              compilationModeConfig: .forNative(),
                                              compilationModeExplicit: false,
                                              iosOutputTarget: nil,
                                              iosGeneratedContextFactories: [],
                                              androidOutputTarget: nil,
                                              webOutputTarget: nil,
                                              cppOutputTarget: nil,
                                              downloadableAssets: false,
                                              downloadableSources: false,
                                              inclusionConfig: .alwaysIncluded,
                                              excludeGlobs: [],
                                              dependencies: [],
                                              stringsConfig: nil,
                                              projectConfig: projectConfig,
                                              outputTarget: .all)
    }

    private func makeCppType(name: String, bundleInfo: CompilationItem.BundleInfo) -> CPPType {
        CPPType(declaration: CPPTypeDeclaration(namespace: "snap::rust_test",
                                                name: name,
                                                symbolType: .class),
                module: bundleInfo,
                includePrefix: nil)
    }

    private func modelProperty(name: String, type: ValdiModelPropertyType) -> ValdiModelProperty {
        ValdiModelProperty(name: name,
                           type: type,
                           comments: nil,
                           omitConstructor: nil,
                           injectableParams: .empty)
    }

    private func source(named filename: String, in sources: [NativeSource]) throws -> String {
        guard let source = sources.first(where: { $0.filename == filename }) else {
            XCTFail("Missing generated source \(filename)")
            return ""
        }

        return try source.file.readString()
    }
}
