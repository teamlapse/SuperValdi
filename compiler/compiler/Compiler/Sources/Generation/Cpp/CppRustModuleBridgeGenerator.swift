//
//  CppRustModuleBridgeGenerator.swift
//  Compiler
//
//  Created by OpenAI on 7/7/26.
//

import Foundation

final class CppRustModuleBridgeGenerator {
    private let bundleInfo: CompilationItem.BundleInfo
    private let cppType: CPPType
    private let exportedModule: ExportedModule
    private let sourceFileName: GeneratedSourceFilename

    init(bundleInfo: CompilationItem.BundleInfo,
         cppType: CPPType,
         exportedModule: ExportedModule,
         sourceFileName: GeneratedSourceFilename) {
        self.bundleInfo = bundleInfo
        self.cppType = cppType
        self.exportedModule = exportedModule
        self.sourceFileName = sourceFileName
    }

    private struct CppPropertyName {
        let methodName: String
    }

    private struct FFIType {
        let name: String
    }

    private struct FFIParameter {
        let type: FFIType
        let prelude: String
        let expression: String
    }

    private enum RustBridgeObservableKind: Equatable {
        case double
        case bool
        case long
        case string
        case bytes
        case handle

        var cppObserverType: String {
            switch self {
            case .double:
                return "ValdiRustDoubleObservableObserver"
            case .bool:
                return "ValdiRustBoolObservableObserver"
            case .long:
                return "ValdiRustLongObservableObserver"
            case .string:
                return "ValdiRustStringObservableObserver"
            case .bytes:
                return "ValdiRustBytesObservableObserver"
            case .handle:
                return "ValdiRustHandleObservableObserver"
            }
        }

        var rustObserverType: String {
            return "valdi_rust::\(cppObserverType)"
        }

        var makeObservableHelperName: String {
            switch self {
            case .double:
                return "valdiRustMakeDoubleBridgeObservable"
            case .bool:
                return "valdiRustMakeBoolBridgeObservable"
            case .long:
                return "valdiRustMakeLongBridgeObservable"
            case .string:
                return "valdiRustMakeStringBridgeObservable"
            case .bytes:
                return "valdiRustMakeBytesBridgeObservable"
            case .handle:
                return "valdiRustMakeHandleBridgeObservable"
            }
        }
    }

    private let rustKeywords: Set<String> = [
        "as", "async", "await", "break", "const", "continue", "crate", "dyn",
        "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
        "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
        "self", "Self", "static", "struct", "super", "trait", "true", "type",
        "unsafe", "use", "where", "while",
    ]

    private func resolvePropertyName(property: ValdiModelProperty, nameAllocator: PropertyNameAllocator) -> CppPropertyName {
        _ = nameAllocator.allocate(property: "_\(property.name)")
        _ = nameAllocator.allocate(property: "get\(property.name.pascalCased)")
        _ = nameAllocator.allocate(property: "set\(property.name.pascalCased)")
        return CppPropertyName(methodName: nameAllocator.allocate(property: property.name).name)
    }

    private func rustSymbolName(methodName: String) -> String {
        return "valdi_rust_\(bundleInfo.name.snakeCased)_\(cppType.declaration.name.snakeCased)_\(methodName.snakeCased)"
    }

    private func rustIdentifier(_ name: String) -> String {
        let identifier = name.snakeCased.replacingOccurrences(of: "-", with: "_")
        if rustKeywords.contains(identifier) {
            return "r#\(identifier)"
        }
        return identifier
    }

    private func rustUserFunctionName(methodName: String) -> String {
        return rustIdentifier(methodName)
    }

    private func validateRustBoundaryMapping(_ mapping: ValdiNodeClassMapping, context: String) throws {
        if mapping.isGenerated || mapping.marshallAsUntyped || mapping.converter != nil {
            return
        }

        throw CompilerError("Rust native modules do not support \(context) type '\(mapping.tsType)'. Use a Valdi-generated native type, a marshallAsUntyped native type, or add a @NativeTypeConverter to an existing Valdi boundary type.")
    }

    private func validateRustBoundaryType(_ type: ValdiModelPropertyType,
                                          context: String,
                                          allowVoid: Bool = false) throws {
        if type.isOptional {
            try validateRustBoundaryType(type.unwrappingOptional, context: context, allowVoid: allowVoid)
            return
        }

        switch type.unwrappingOptional {
        case .string, .double, .bool, .long, .bytes, .any:
            return
        case .void:
            if allowVoid {
                return
            }
            throw CompilerError("Rust native modules do not support void for \(context)")
        case .array(let elementType):
            try validateRustBoundaryType(elementType, context: "\(context) array element")
        case .map(let keyType, let valueType):
            try validateRustBoundaryType(keyType, context: "\(context) map key")
            try validateRustBoundaryType(valueType, context: "\(context) map value")
        case .function(let parameters, let returnType, _, _, _):
            for parameter in parameters {
                try validateRustBoundaryType(parameter.type, context: "\(context) function parameter '\(parameter.name)'")
            }
            try validateRustBoundaryType(returnType, context: "\(context) function return", allowVoid: true)
        case .object(let mapping):
            try validateRustBoundaryMapping(mapping, context: context)
        case .genericObject(let mapping, let typeArguments):
            try validateRustBoundaryMapping(mapping, context: context)
            for (index, typeArgument) in typeArguments.enumerated() {
                try validateRustBoundaryType(typeArgument, context: "\(context) generic argument \(index)")
            }
        case .promise(let typeArgument):
            try validateRustBoundaryType(typeArgument, context: "\(context) promise value")
        case .enum(let mapping):
            try validateRustBoundaryMapping(mapping, context: context)
        case .genericTypeParameter(let name):
            throw CompilerError("Rust native modules require concrete boundary types; generic type parameter '\(name)' is not supported for \(context)")
        case .nullable(let innerType):
            try validateRustBoundaryType(innerType, context: context, allowVoid: allowVoid)
        }
    }

    private func bridgeObservableReturn(for type: ValdiModelPropertyType, context: String) throws -> RustBridgeObservableKind? {
        guard !type.isOptional else {
            return nil
        }

        switch type.unwrappingOptional {
        case .genericObject(let mapping, let typeArguments):
            guard mapping.tsType == "BridgeObservable", typeArguments.count == 1 else {
                return nil
            }

            let valueType = typeArguments[0]
            try validateRustBoundaryType(valueType, context: "\(context) BridgeObservable value")

            switch valueType.unwrappingOptional {
            case .double:
                return .double
            case .bool:
                return .bool
            case .long:
                return .long
            case .string:
                return .string
            case .bytes:
                return .bytes
            case .nullable:
                return .handle
            default:
                return .handle
            }
        default:
            return nil
        }
    }

    private func rustFFIParameterType(for type: ValdiModelPropertyType) -> String {
        if type.isOptional {
            return "valdi_rust::ValdiRustHandle"
        }

        switch type.unwrappingOptional {
        case .double:
            return "f64"
        case .bool:
            return "bool"
        case .long:
            return "i64"
        case .string:
            return "valdi_rust::ValdiRustStringView"
        case .bytes:
            return "valdi_rust::ValdiRustBytesView"
        default:
            return "valdi_rust::ValdiRustHandle"
        }
    }

    private func rustUserArgumentExpression(type: ValdiModelPropertyType, name: String) -> String {
        if type.isOptional {
            return name
        }

        switch type.unwrappingOptional {
        case .string:
            return "\(name).to_string()"
        case .bytes:
            return "\(name).to_vec()"
        default:
            return name
        }
    }

    private func rustUserReturnExpression(returnType: ValdiModelPropertyType, callExpression: String) -> String {
        switch returnType.unwrappingOptional {
        case .string:
            if returnType.isOptional {
                return callExpression
            }
            return "valdi_rust::ValdiRustOwnedString::from_value(\(callExpression))"
        case .bytes:
            if returnType.isOptional {
                return callExpression
            }
            return "valdi_rust::ValdiRustOwnedBytes::from_value(\(callExpression))"
        default:
            return callExpression
        }
    }

    private func rustFFIReturnType(for type: ValdiModelPropertyType) -> String? {
        switch type.unwrappingOptional {
        case .void:
            return nil
        case .string:
            return type.isOptional ? "valdi_rust::ValdiRustHandle" : "valdi_rust::ValdiRustOwnedString"
        case .bytes:
            return type.isOptional ? "valdi_rust::ValdiRustHandle" : "valdi_rust::ValdiRustOwnedBytes"
        default:
            return rustFFIParameterType(for: type)
        }
    }

    private func ffiType(for type: ValdiModelPropertyType) -> FFIType {
        if type.isOptional {
            return FFIType(name: "ValdiRustHandle")
        }

        switch type.unwrappingOptional {
        case .double:
            return FFIType(name: "double")
        case .bool:
            return FFIType(name: "bool")
        case .long:
            return FFIType(name: "int64_t")
        case .void:
            return FFIType(name: "void")
        case .string:
            return FFIType(name: "ValdiRustStringView")
        case .bytes:
            return FFIType(name: "ValdiRustBytesView")
        default:
            return FFIType(name: "ValdiRustHandle")
        }
    }

    private func ffiReturnType(for type: ValdiModelPropertyType) -> FFIType {
        switch type.unwrappingOptional {
        case .string:
            return FFIType(name: type.isOptional ? "ValdiRustHandle" : "ValdiRustOwnedString")
        case .bytes:
            return FFIType(name: type.isOptional ? "ValdiRustHandle" : "ValdiRustOwnedBytes")
        default:
            return ffiType(for: type)
        }
    }

    private func makeFFIParameter(type: ValdiModelPropertyType, cppTypeName: String, name: String) -> FFIParameter {
        let rustName = "\(name)Rust"
        switch type.unwrappingOptional {
        case .double, .bool, .long:
            if type.isOptional {
                return makeHandleFFIParameter(cppTypeName: cppTypeName, name: name, rustName: rustName)
            }
            return FFIParameter(type: ffiType(for: type), prelude: "", expression: name)
        case .string:
            if type.isOptional {
                return makeHandleFFIParameter(cppTypeName: cppTypeName, name: name, rustName: rustName)
            }
            let viewName = "\(name)View"
            return FFIParameter(
                type: ffiType(for: type),
                prelude: """
                auto \(viewName) = \(name).toStringView();
                ValdiRustStringView \(rustName){\(viewName).data(), \(viewName).size()};
                """,
                expression: rustName
            )
        case .bytes:
            if type.isOptional {
                return makeHandleFFIParameter(cppTypeName: cppTypeName, name: name, rustName: rustName)
            }
            return FFIParameter(
                type: ffiType(for: type),
                prelude: "ValdiRustBytesView \(rustName){\(name).data(), \(name).size()};",
                expression: rustName
            )
        default:
            return makeHandleFFIParameter(cppTypeName: cppTypeName, name: name, rustName: rustName)
        }
    }

    private func makeHandleFFIParameter(cppTypeName: String, name: String, rustName: String) -> FFIParameter {
        return FFIParameter(
            type: FFIType(name: "ValdiRustHandle"),
            prelude: """
            auto \(rustName)Box = Valdi::makeShared<ValdiRustBridgeBox<\(cppTypeName)>>(\(name));
            ValdiRustHandle \(rustName){Valdi::unsafeBridgeCast(\(rustName)Box.get()), valdiRustBridgeRetain, valdiRustBridgeRelease};
            """,
            expression: rustName
        )
    }

    private func returnStatement(returnType: ValdiModelPropertyType,
                                 cppTypeName: String,
                                 callExpression: String) -> String {
        switch returnType.unwrappingOptional {
        case .void:
            return "\(callExpression);\n"
        case .double, .bool, .long:
            if returnType.isOptional {
                return "return valdiRustBridgeTakeBoxedValue<\(cppTypeName)>(\(callExpression));\n"
            }
            return "return \(callExpression);\n"
        case .string:
            if returnType.isOptional {
                return "return valdiRustBridgeTakeBoxedValue<\(cppTypeName)>(\(callExpression));\n"
            }
            return "return valdiRustStringToStringBox(\(callExpression));\n"
        case .bytes:
            if returnType.isOptional {
                return "return valdiRustBridgeTakeBoxedValue<\(cppTypeName)>(\(callExpression));\n"
            }
            return "return valdiRustBytesToBytesView(\(callExpression));\n"
        default:
            return "return valdiRustBridgeTakeBoxedValue<\(cppTypeName)>(\(callExpression));\n"
        }
    }

    private func writeCommonRuntime(to writer: CodeWriter) {
        writer.appendBody("""
            struct ValdiRustStringView {
                const char *data;
                size_t len;
            };

            struct ValdiRustOwnedString {
                const char *data;
                size_t len;
                void (*free)(const char *data, size_t len);
            };

            struct ValdiRustBytesView {
                const uint8_t *data;
                size_t len;
            };

            struct ValdiRustOwnedBytes {
                const uint8_t *data;
                size_t len;
                void (*free)(const uint8_t *data, size_t len);
            };

            struct ValdiRustHandle {
                void *ptr;
                void (*retain)(void *ptr);
                void (*release)(void *ptr);
            };

            struct ValdiRustObservableSubscription {
                void *ptr;
                void (*unsubscribe)(void *ptr);
            };

            struct ValdiRustDoubleObservableObserver {
                void *context;
                void (*next)(void *context, double value);
                void (*complete)(void *context);
                void (*release)(void *context);
            };

            struct ValdiRustBoolObservableObserver {
                void *context;
                void (*next)(void *context, bool value);
                void (*complete)(void *context);
                void (*release)(void *context);
            };

            struct ValdiRustLongObservableObserver {
                void *context;
                void (*next)(void *context, int64_t value);
                void (*complete)(void *context);
                void (*release)(void *context);
            };

            struct ValdiRustStringObservableObserver {
                void *context;
                void (*next)(void *context, ValdiRustStringView value);
                void (*complete)(void *context);
                void (*release)(void *context);
            };

            struct ValdiRustBytesObservableObserver {
                void *context;
                void (*next)(void *context, ValdiRustBytesView value);
                void (*complete)(void *context);
                void (*release)(void *context);
            };

            struct ValdiRustHandleObservableObserver {
                void *context;
                void (*next)(void *context, ValdiRustHandle value);
                void (*complete)(void *context);
                void (*release)(void *context);
            };

            template<typename T>
            class ValdiRustBridgeBox final : public Valdi::SimpleRefCountable {
            public:
                explicit ValdiRustBridgeBox(const T &value): value(value) {}
                explicit ValdiRustBridgeBox(T &&value): value(std::move(value)) {}

                T value;
            };

            [[maybe_unused]] static void valdiRustBridgeRetain(void *ptr) {
                Valdi::unsafeBridgeRetain(reinterpret_cast<Valdi::RefCountable *>(ptr));
            }

            [[maybe_unused]] static void valdiRustBridgeRelease(void *ptr) {
                Valdi::unsafeBridgeRelease(ptr);
            }

            [[maybe_unused]] static Valdi::StringBox valdiRustStringToStringBox(ValdiRustOwnedString string) {
                auto out = Valdi::StringBox::fromString(std::string_view(string.data, string.len));
                if (string.free != nullptr) {
                    string.free(string.data, string.len);
                }
                return out;
            }

            [[maybe_unused]] static Valdi::StringBox valdiRustStringViewToStringBox(ValdiRustStringView string) {
                return Valdi::StringBox::fromString(std::string_view(string.data, string.len));
            }

            [[maybe_unused]] static Valdi::BytesView valdiRustBytesToBytesView(ValdiRustOwnedBytes bytes) {
                auto out = Valdi::makeShared<Valdi::Bytes>();
                out->assignData(reinterpret_cast<const Valdi::Byte *>(bytes.data), bytes.len);
                if (bytes.free != nullptr) {
                    bytes.free(bytes.data, bytes.len);
                }
                return Valdi::BytesView(out);
            }

            [[maybe_unused]] static Valdi::BytesView valdiRustBytesViewToBytesView(ValdiRustBytesView bytes) {
                auto out = Valdi::makeShared<Valdi::Bytes>();
                out->assignData(reinterpret_cast<const Valdi::Byte *>(bytes.data), bytes.len);
                return Valdi::BytesView(out);
            }

            template<typename T>
            [[maybe_unused]] static T valdiRustBridgeTakeBoxedValue(ValdiRustHandle handle) {
                auto box = Valdi::unsafeBridgeTransfer<ValdiRustBridgeBox<T>>(handle.ptr);
                return std::move(box->value);
            }

            template<typename OnEventFn>
            [[maybe_unused]] static void valdiRustDoubleObservableNext(void *context, double value) {
                auto *box = Valdi::unsafeBridgeUnretained<ValdiRustBridgeBox<OnEventFn>>(context);
                box->value(::snap::valdi_modules::bridge_observables::BridgeObserverEvent::NEXT,
                           std::nullopt,
                           std::optional<double>(value),
                           nullptr);
            }

            template<typename OnEventFn>
            [[maybe_unused]] static void valdiRustBoolObservableNext(void *context, bool value) {
                auto *box = Valdi::unsafeBridgeUnretained<ValdiRustBridgeBox<OnEventFn>>(context);
                box->value(::snap::valdi_modules::bridge_observables::BridgeObserverEvent::NEXT,
                           std::nullopt,
                           std::optional<bool>(value),
                           nullptr);
            }

            template<typename OnEventFn>
            [[maybe_unused]] static void valdiRustLongObservableNext(void *context, int64_t value) {
                auto *box = Valdi::unsafeBridgeUnretained<ValdiRustBridgeBox<OnEventFn>>(context);
                box->value(::snap::valdi_modules::bridge_observables::BridgeObserverEvent::NEXT,
                           std::nullopt,
                           std::optional<int64_t>(value),
                           nullptr);
            }

            template<typename OnEventFn>
            [[maybe_unused]] static void valdiRustStringObservableNext(void *context, ValdiRustStringView value) {
                auto *box = Valdi::unsafeBridgeUnretained<ValdiRustBridgeBox<OnEventFn>>(context);
                box->value(::snap::valdi_modules::bridge_observables::BridgeObserverEvent::NEXT,
                           std::nullopt,
                           std::optional<Valdi::StringBox>(valdiRustStringViewToStringBox(value)),
                           nullptr);
            }

            template<typename OnEventFn>
            [[maybe_unused]] static void valdiRustBytesObservableNext(void *context, ValdiRustBytesView value) {
                auto *box = Valdi::unsafeBridgeUnretained<ValdiRustBridgeBox<OnEventFn>>(context);
                box->value(::snap::valdi_modules::bridge_observables::BridgeObserverEvent::NEXT,
                           std::nullopt,
                           std::optional<Valdi::BytesView>(valdiRustBytesViewToBytesView(value)),
                           nullptr);
            }

            template<typename OnEventFn, typename T>
            [[maybe_unused]] static void valdiRustHandleObservableNext(void *context, ValdiRustHandle value) {
                auto *box = Valdi::unsafeBridgeUnretained<ValdiRustBridgeBox<OnEventFn>>(context);
                box->value(::snap::valdi_modules::bridge_observables::BridgeObserverEvent::NEXT,
                           std::nullopt,
                           std::optional<T>(valdiRustBridgeTakeBoxedValue<T>(value)),
                           nullptr);
            }

            template<typename OnEventFn>
            [[maybe_unused]] static void valdiRustObservableComplete(void *context) {
                auto *box = Valdi::unsafeBridgeUnretained<ValdiRustBridgeBox<OnEventFn>>(context);
                box->value(::snap::valdi_modules::bridge_observables::BridgeObserverEvent::COMPLETE,
                           std::nullopt,
                           std::nullopt,
                           nullptr);
            }

            [[maybe_unused]] static void valdiRustObservableRelease(void *context) {
                Valdi::unsafeBridgeRelease(context);
            }

            template<typename Observable, typename RustObserver, typename SubscribeRust, typename NextFn>
            [[maybe_unused]] static Observable valdiRustMakeBridgeObservable(SubscribeRust subscribeRust, NextFn nextFn) {
                using OnEventFn = typename Observable::SubscribeOnEventFn;
                using SubscriptionFn = typename Observable::SubscribeOnEventSubscriptionFn;

                return Observable([subscribeRust, nextFn](OnEventFn onEvent) {
                    auto onEventBox = Valdi::makeShared<ValdiRustBridgeBox<OnEventFn>>(std::move(onEvent));
                    RustObserver observer{
                        Valdi::unsafeBridgeRetain(onEventBox.get()),
                        nextFn,
                        valdiRustObservableComplete<OnEventFn>,
                        valdiRustObservableRelease
                    };
                    auto subscription = subscribeRust(observer);
                    SubscriptionFn unsubscribe = [subscription]() mutable {
                        if (subscription.unsubscribe != nullptr) {
                            subscription.unsubscribe(subscription.ptr);
                            subscription.ptr = nullptr;
                            subscription.unsubscribe = nullptr;
                        }
                    };
                    onEventBox->value(::snap::valdi_modules::bridge_observables::BridgeObserverEvent::RECEIVE_SUBSCRIPTION,
                                      std::optional<SubscriptionFn>(std::move(unsubscribe)),
                                      std::nullopt,
                                      nullptr);
                });
            }

            template<typename Observable, typename SubscribeRust>
            [[maybe_unused]] static Observable valdiRustMakeDoubleBridgeObservable(SubscribeRust subscribeRust) {
                using OnEventFn = typename Observable::SubscribeOnEventFn;
                return valdiRustMakeBridgeObservable<Observable, ValdiRustDoubleObservableObserver>(subscribeRust, valdiRustDoubleObservableNext<OnEventFn>);
            }

            template<typename Observable, typename SubscribeRust>
            [[maybe_unused]] static Observable valdiRustMakeBoolBridgeObservable(SubscribeRust subscribeRust) {
                using OnEventFn = typename Observable::SubscribeOnEventFn;
                return valdiRustMakeBridgeObservable<Observable, ValdiRustBoolObservableObserver>(subscribeRust, valdiRustBoolObservableNext<OnEventFn>);
            }

            template<typename Observable, typename SubscribeRust>
            [[maybe_unused]] static Observable valdiRustMakeLongBridgeObservable(SubscribeRust subscribeRust) {
                using OnEventFn = typename Observable::SubscribeOnEventFn;
                return valdiRustMakeBridgeObservable<Observable, ValdiRustLongObservableObserver>(subscribeRust, valdiRustLongObservableNext<OnEventFn>);
            }

            template<typename Observable, typename SubscribeRust>
            [[maybe_unused]] static Observable valdiRustMakeStringBridgeObservable(SubscribeRust subscribeRust) {
                using OnEventFn = typename Observable::SubscribeOnEventFn;
                return valdiRustMakeBridgeObservable<Observable, ValdiRustStringObservableObserver>(subscribeRust, valdiRustStringObservableNext<OnEventFn>);
            }

            template<typename Observable, typename SubscribeRust>
            [[maybe_unused]] static Observable valdiRustMakeBytesBridgeObservable(SubscribeRust subscribeRust) {
                using OnEventFn = typename Observable::SubscribeOnEventFn;
                return valdiRustMakeBridgeObservable<Observable, ValdiRustBytesObservableObserver>(subscribeRust, valdiRustBytesObservableNext<OnEventFn>);
            }

            template<typename Observable, typename Value, typename SubscribeRust>
            [[maybe_unused]] static Observable valdiRustMakeHandleBridgeObservable(SubscribeRust subscribeRust) {
                using OnEventFn = typename Observable::SubscribeOnEventFn;
                return valdiRustMakeBridgeObservable<Observable, ValdiRustHandleObservableObserver>(subscribeRust, valdiRustHandleObservableNext<OnEventFn, Value>);
            }

            """)
    }

    func write() throws -> [NativeSource] {
        let moduleFactoryClassName = "\(cppType.declaration.name)Factory"
        let moduleFactoryCppType = CPPType(declaration: CPPTypeDeclaration(namespace: self.cppType.declaration.namespace,
                                                                           name: moduleFactoryClassName,
                                                                           symbolType: .class),
                                           module: bundleInfo,
                                           includePrefix: self.cppType.includePrefix)
        let modelNamespace = cppType.declaration.namespace.appending(component: cppType.declaration.name)
        let typeGenerator = CppCodeGenerator(namespace: cppType.declaration.namespace,
                                             selfIncludePath: moduleFactoryCppType.includePath,
                                             namespaceResolver: CppCodeGeneratorSingleNamespaceResolver(classNamespace: modelNamespace))
        let nameAllocator = PropertyNameAllocator.forCpp()

        let generator = CppFileGenerator(namespace: cppType.declaration.namespace, isHeader: false)
        generator.includeSection.addInclude(path: moduleFactoryCppType.includePath)
        generator.includeSection.addInclude(path: "valdi_core/cpp/Utils/Bytes.hpp")
        generator.includeSection.addInclude(path: "valdi_core/cpp/Utils/StringBox.hpp")
        generator.includeSection.addSystemInclude(path: "cstdint")
        generator.includeSection.addSystemInclude(path: "cstddef")
        generator.includeSection.addSystemInclude(path: "optional")
        generator.includeSection.addSystemInclude(path: "string_view")
        generator.includeSection.addSystemInclude(path: "utility")

        generator.body.appendBody(FileHeaderCommentGenerator.generateComment(sourceFilename: sourceFileName, additionalComments: """
            Generated Rust bridge for \(cppType.declaration.fullTypeName).
            Rust symbols are derived from the @ExportModule TypeScript declaration.

            """))

        writeCommonRuntime(to: generator.body)

        var methodImplementations = ""
        var externDeclarations = ""
        var rustAdapterFunctions = ""

        for property in exportedModule.model.properties {
            switch property.type {
            case .function(let parameters, let returnType, _, _, _):
                let methodTypeParser = try typeGenerator.getTypeParser(type: property.type, namePaths: [property.name], nameAllocator: nameAllocator)
                let propertyName = resolvePropertyName(property: property, nameAllocator: nameAllocator)
                let rustSymbol = rustSymbolName(methodName: propertyName.methodName)
                guard let cppMethod = methodTypeParser.method else {
                    throw CompilerError("Could not generate Rust bridge method for \(property.name)")
                }

                let scopedNameAllocator = nameAllocator.scoped()
                let parameterNames = cppMethod.parameters.map { scopedNameAllocator.allocate(property: $0.name).name }
                let cppParameters = cppMethod.parameters.indices.map { index -> String in
                    let parameterName = parameterNames[index]
                    let parameterType = cppMethod.parameters[index].typeNameResolver.resolve(cppType.declaration.namespace)
                    return "\(parameterType) \(parameterName)"
                }
                let ffiParameters = parameters.indices.map { index -> FFIParameter in
                    let cppParameterType = cppMethod.parameters[index].typeNameResolver.resolve(cppType.declaration.namespace)
                    return makeFFIParameter(type: parameters[index].type, cppTypeName: cppParameterType, name: parameterNames[index])
                }
                let ffiParameterDeclarations = zip(ffiParameters, parameterNames).map {
                    "\($0.0.type.name) \($0.1)"
                }.joined(separator: ", ")
                let rustCallArguments = ffiParameters.map { $0.expression }.joined(separator: ", ")
                let returnCppType = cppMethod.returnTypeNameResolver.resolve(cppType.declaration.namespace)
                let returnFFIType = ffiReturnType(for: returnType)
                let returnTypeName = cppMethod.returnTypeNameResolver.resolve(cppType.declaration.namespace)
                let rustUserFunctionName = rustUserFunctionName(methodName: propertyName.methodName)
                let rustParameterNames = parameters.map { rustIdentifier($0.name) }
                let rustParameterDeclarations = zip(parameters, rustParameterNames).map {
                    "\($0.1): \(rustFFIParameterType(for: $0.0.type))"
                }.joined(separator: ", ")
                let rustUserArguments = zip(parameters, rustParameterNames).map {
                    rustUserArgumentExpression(type: $0.0.type, name: $0.1)
                }.joined(separator: ", ")

                try validateRustBoundaryType(property.type, context: "exported module function '\(property.name)'", allowVoid: true)

                if let bridgeObservableReturn = try bridgeObservableReturn(for: returnType, context: "exported module function '\(property.name)'") {
                    let rustObserverParameterDeclaration = "observer: \(bridgeObservableReturn.rustObserverType)"
                    let rustObservableParameterDeclarations = (rustParameterDeclarations.isEmpty
                                                               ? rustObserverParameterDeclaration
                                                               : "\(rustParameterDeclarations), \(rustObserverParameterDeclaration)")
                    let ffiObservableParameterDeclarations = ([ffiParameterDeclarations].filter { !$0.isEmpty } + ["\(bridgeObservableReturn.cppObserverType) observer"]).joined(separator: ", ")
                    let rustObservableCallArguments = ([rustCallArguments].filter { !$0.isEmpty } + ["observer"]).joined(separator: ", ")
                    externDeclarations += "ValdiRustObservableSubscription \(rustSymbol)(\(ffiObservableParameterDeclarations));\n"
                    rustAdapterFunctions += """

                        #[unsafe(no_mangle)]
                        pub extern "C" fn \(rustSymbol)(\(rustObservableParameterDeclarations)) -> valdi_rust::ValdiRustObservableSubscription {
                            valdi_rust::subscribe(valdi_rust_module::\(rustUserFunctionName)(\(rustUserArguments)), observer)
                        }

                    """
                    let observableFactoryExpression: String
                    if bridgeObservableReturn == .handle {
                        guard case .genericObject(_, let typeArguments) = returnType.unwrappingOptional,
                              typeArguments.count == 1 else {
                            throw CompilerError("Could not resolve Rust BridgeObservable value type for \(property.name)")
                        }
                        let valueTypeParser = try typeGenerator.getTypeParser(type: typeArguments[0],
                                                                               namePaths: [property.name, "value"],
                                                                               nameAllocator: nameAllocator.scoped())
                        let valueTypeName = valueTypeParser.typeNameResolver.resolve(cppType.declaration.namespace)
                        observableFactoryExpression = "\(bridgeObservableReturn.makeObservableHelperName)<\(returnTypeName), \(valueTypeName)>"
                    } else {
                        observableFactoryExpression = "\(bridgeObservableReturn.makeObservableHelperName)<\(returnTypeName)>"
                    }
                    let observableFFIPreludeBlock = ffiParameters.map { $0.prelude }
                        .filter { !$0.isEmpty }
                        .map { $0.indented }
                        .joined(separator: "\n")
                    methodImplementations += """

                        \(returnTypeName) \(propertyName.methodName)(\(cppParameters.joined(separator: ", "))) final {
                            auto subscribeRust = [=](\(bridgeObservableReturn.cppObserverType) observer) -> ValdiRustObservableSubscription {
                                \(observableFFIPreludeBlock)
                                return \(rustSymbol)(\(rustObservableCallArguments));
                            };
                            return \(observableFactoryExpression)(subscribeRust);
                        }

                    """
                    continue
                }

                externDeclarations += "\(returnFFIType.name) \(rustSymbol)(\(ffiParameterDeclarations));\n"
                if let rustReturnType = rustFFIReturnType(for: returnType) {
                    rustAdapterFunctions += """

                        #[unsafe(no_mangle)]
                        pub extern "C" fn \(rustSymbol)(\(rustParameterDeclarations)) -> \(rustReturnType) {
                            \(rustUserReturnExpression(returnType: returnType, callExpression: "valdi_rust_module::\(rustUserFunctionName)(\(rustUserArguments))"))
                        }

                    """
                } else {
                    rustAdapterFunctions += """

                        #[unsafe(no_mangle)]
                        pub extern "C" fn \(rustSymbol)(\(rustParameterDeclarations)) {
                            valdi_rust_module::\(rustUserFunctionName)(\(rustUserArguments));
                        }

                    """
                }
                methodImplementations += """

                    \(returnTypeName) \(propertyName.methodName)(\(cppParameters.joined(separator: ", "))) final {
                \(ffiParameters.map { $0.prelude }.filter { !$0.isEmpty }.map { $0.indented }.joined(separator: "\n"))
                        \(returnStatement(returnType: returnType, cppTypeName: returnCppType, callExpression: "\(rustSymbol)(\(rustCallArguments))").indented)
                    }

                """
            default:
                try validateRustBoundaryType(property.type, context: "exported module property '\(property.name)'")

                let typeParser = try typeGenerator.getTypeParser(type: property.type, namePaths: [property.name], nameAllocator: nameAllocator)
                let propertyName = resolvePropertyName(property: property, nameAllocator: nameAllocator)
                let rustSymbol = rustSymbolName(methodName: propertyName.methodName)
                let returnCppType = typeParser.typeNameResolver.resolve(cppType.declaration.namespace)
                let returnFFIType = ffiReturnType(for: property.type)
                let rustUserFunctionName = rustUserFunctionName(methodName: propertyName.methodName)
                externDeclarations += "\(returnFFIType.name) \(rustSymbol)();\n"
                if let rustReturnType = rustFFIReturnType(for: property.type) {
                    rustAdapterFunctions += """

                        #[unsafe(no_mangle)]
                        pub extern "C" fn \(rustSymbol)() -> \(rustReturnType) {
                            \(rustUserReturnExpression(returnType: property.type, callExpression: "valdi_rust_module::\(rustUserFunctionName)()"))
                        }

                    """
                } else {
                    rustAdapterFunctions += """

                        #[unsafe(no_mangle)]
                        pub extern "C" fn \(rustSymbol)() {
                            valdi_rust_module::\(rustUserFunctionName)();
                        }

                    """
                }
                methodImplementations += """

                    \(returnCppType) \(propertyName.methodName)() final {
                        \(returnStatement(returnType: property.type, cppTypeName: returnCppType, callExpression: "\(rustSymbol)()").indented)
                    }

                """
            }
        }

        generator.body.appendBody("""
            extern "C" {
            \(externDeclarations.indented)
            }

            class \(cppType.declaration.name)RustImpl final : public \(cppType.declaration.name) {
            public:
                \(cppType.declaration.name)RustImpl() = default;
                ~\(cppType.declaration.name)RustImpl() override = default;
            \(methodImplementations)
            };

            class \(cppType.declaration.name)RustFactoryImpl final : public \(moduleFactoryClassName) {
            public:
                \(cppType.declaration.name)RustFactoryImpl() = default;
                ~\(cppType.declaration.name)RustFactoryImpl() override = default;

                Valdi::Ref<\(cppType.declaration.name)> onLoadModule() final {
                    return Valdi::makeShared<\(cppType.declaration.name)RustImpl>();
                }
            };

            static auto kRegister\(cppType.declaration.name)RustModule = Valdi::RegisterModuleFactory::registerTyped<\(cppType.declaration.name)RustFactoryImpl>();
            """)

        return [
            NativeSource(relativePath: cppType.includeDir,
                         filename: "\(cppType.declaration.name)RustBridge.cpp",
                         file: .data(try generator.content.indented.utf8Data()),
                         groupingIdentifier: "\(bundleInfo.name).rust_bridge.cpp",
                         groupingPriority: 0),
            NativeSource(relativePath: cppType.includeDir,
                         filename: "\(bundleInfo.name).rust_bridge.rs",
                         file: .data(try rustAdapterSource(functions: rustAdapterFunctions).utf8Data()),
                         groupingIdentifier: "\(bundleInfo.name).rust_bridge.rs",
                         groupingPriority: 0)
        ]
    }

    private func rustAdapterSource(functions: String) -> String {
        return """
            // Generated Rust adapter for \(cppType.declaration.fullTypeName).
            // Rust symbols are derived from the @ExportModule TypeScript declaration.

            #[allow(dead_code)]
            pub mod valdi_rust {
                include!(env!("VALDI_RUST_SUPPORT_PATH"));
            }

            #[allow(dead_code)]
            mod valdi_rust_module {
                #[allow(unused_imports)]
                use crate::valdi_rust::*;

                include!(env!("VALDI_RUST_USER_ROOT_PATH"));
            }
            \(functions)
            """.indented
    }
}
