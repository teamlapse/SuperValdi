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

    func testGeneratedEnumHelpersAreEmitted() throws {
        let bundleInfo = try makeRustTestBundleInfo()
        let moduleCppType = makeCppType(name: "RustTestModule", bundleInfo: bundleInfo)
        let enumCppType = makeCppType(name: "CounterKind", bundleInfo: bundleInfo, symbolType: .enum)
        let observableCppType = CPPType(declaration: CPPTypeDeclaration(namespace: "snap::valdi_modules::bridge_observables",
                                                                        name: "BridgeObservable",
                                                                        symbolType: .class),
                                        module: bundleInfo,
                                        includePrefix: nil)
        let exportedEnum = ExportedEnum(iosType: nil,
                                        androidTypeName: nil,
                                        cppType: enumCppType,
                                        cases: .enum([
                                            EnumCase(name: "idle", value: 10, comments: nil),
                                            EnumCase(name: "running", value: 20, comments: nil),
                                        ]),
                                        comments: nil)
        var enumMapping = ValdiNodeClassMapping(tsType: "CounterKind",
                                                iosType: nil,
                                                androidClassName: nil,
                                                cppType: enumCppType,
                                                kind: .enum)
        enumMapping.isGenerated = true
        var observableMapping = ValdiNodeClassMapping(tsType: "BridgeObservable",
                                                      iosType: nil,
                                                      androidClassName: nil,
                                                      cppType: observableCppType,
                                                      kind: .class)
        observableMapping.marshallAsUntyped = true

        let enumType = ValdiModelPropertyType.enum(enumMapping)
        let observableEnumType = ValdiModelPropertyType.genericObject(observableMapping, typeArguments: [enumType])
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
                                        modelProperty(name: "echoKind",
                                                      type: .function(parameters: [
                                                        modelProperty(name: "kind", type: enumType),
                                                      ],
                                                      returnType: enumType,
                                                      isSingleCall: false,
                                                      shouldCallOnWorkerThread: false,
                                                      allowSyncCall: false)),
                                        modelProperty(name: "kindAsync",
                                                      type: .function(parameters: [],
                                                                      returnType: .promise(typeArgument: enumType),
                                                                      isSingleCall: false,
                                                                      shouldCallOnWorkerThread: false,
                                                                      allowSyncCall: false)),
                                        modelProperty(name: "kindStream",
                                                      type: .function(parameters: [],
                                                                      returnType: observableEnumType,
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
                                                     generatedEnumsByTypeName: [
                                                        "CounterKind": exportedEnum,
                                                        enumCppType.declaration.fullTypeName: exportedEnum,
                                                     ])

        let sources = try generator.write()
        let rustSource = try source(named: "rust_test.rust_bridge.rs", in: sources)
        let cppSource = try source(named: "RustTestModuleRustBridge.cpp", in: sources)

        XCTAssertTrue(rustSource.contains("pub enum CounterKind"))
        XCTAssertTrue(rustSource.contains("Idle = 0,"))
        XCTAssertTrue(rustSource.contains("Running = 1,"))
        XCTAssertTrue(rustSource.contains("pub fn from_ffi(value: i64) -> Self"))
        XCTAssertTrue(rustSource.contains("pub fn into_ffi(self) -> i64"))
        XCTAssertTrue(rustSource.contains("Self::Idle => 10,"))
        XCTAssertTrue(rustSource.contains("impl crate::valdi_rust::ValdiRustPromiseValue for CounterKind"))
        XCTAssertTrue(rustSource.contains("impl crate::valdi_rust::ValdiRustObservableValue for CounterKind"))
        XCTAssertTrue(rustSource.contains("pub extern \"C\" fn valdi_rust_rust_test_rust_test_module_echo_kind(kind: i64) -> i64"))
        XCTAssertTrue(rustSource.contains("valdi_rust_module::echo_kind(valdi_rust_module::CounterKind::from_ffi(kind)).into_ffi()"))
        XCTAssertTrue(rustSource.contains("pub extern \"C\" fn valdi_rust_rust_test_rust_test_module_kind_stream(observer: valdi_rust::ValdiRustLongObservableObserver) -> valdi_rust::ValdiRustObservableSubscription"))

        XCTAssertTrue(cppSource.contains("return static_cast<CounterKind>(valdi_rust_rust_test_rust_test_module_echo_kind(static_cast<int64_t>(kind)));"))
        XCTAssertTrue(cppSource.contains("resolver.resolveLong = valdiRustPromiseResolveEnum<CounterKind>;"))
        XCTAssertTrue(cppSource.contains("valdiRustMakeEnumBridgeObservable<"))
        XCTAssertTrue(cppSource.contains(", CounterKind>(subscribeRust);"))
        XCTAssertTrue(cppSource.contains("std::optional<T>(static_cast<T>(value))"))
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

    private func makeCppType(name: String,
                             bundleInfo: CompilationItem.BundleInfo,
                             symbolType: CPPTypeDeclaration.SymbolType = .class) -> CPPType {
        CPPType(declaration: CPPTypeDeclaration(namespace: "snap::rust_test",
                                                name: name,
                                                symbolType: symbolType),
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
