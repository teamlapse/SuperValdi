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

    private struct RustHandleTypeDefinition: Hashable {
        let name: String
        let typeParameterCount: Int
    }

    private struct RustCallbackDefinition {
        let wrapperName: String
        let symbolName: String
        let parameterNames: [String]
        let parameterTypes: [ValdiModelPropertyType]
        let returnType: ValdiModelPropertyType
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

    private func rustTypeIdentifier(_ name: String) -> String {
        let sanitized = name.replacingOccurrences(of: "[^A-Za-z0-9_]", with: "_", options: .regularExpression)
        var identifier = sanitized.pascalCased
        if identifier.isEmpty {
            identifier = "ValdiRustType"
        }
        if identifier.first?.isNumber == true {
            identifier = "Rust\(identifier)"
        }
        if rustKeywords.contains(identifier) {
            identifier = "Rust\(identifier)"
        }
        return identifier
    }

    private func rustUserFunctionName(methodName: String) -> String {
        return rustIdentifier(methodName)
    }

    private func rustCallbackWrapperName(methodName: String, parameterName: String) -> String {
        return rustTypeIdentifier("\(methodName)_\(parameterName)_callback")
    }

    private func rustCallbackSymbolName(methodName: String, parameterName: String) -> String {
        return "\(rustSymbolName(methodName: methodName))_\(parameterName.snakeCased)_call"
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

    private func isBridgeObservableMapping(_ mapping: ValdiNodeClassMapping) -> Bool {
        if mapping.tsType == "BridgeObservable" {
            return true
        }

        return mapping.cppType?.declaration.name == "BridgeObservable"
    }

    private func isPromiseType(_ type: ValdiModelPropertyType) -> Bool {
        guard !type.isOptional else {
            return false
        }

        if case .promise = type.unwrappingOptional {
            return true
        }

        return false
    }

    private func bridgeObservableReturn(for type: ValdiModelPropertyType, context: String) throws -> RustBridgeObservableKind? {
        guard !type.isOptional else {
            return nil
        }

        switch type.unwrappingOptional {
        case .genericObject(let mapping, let typeArguments):
            guard isBridgeObservableMapping(mapping), typeArguments.count == 1 else {
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

    private func isRustHandleBackedUserType(_ type: ValdiModelPropertyType) -> Bool {
        if type.isOptional {
            return true
        }

        switch type.unwrappingOptional {
        case .double, .bool, .long, .string, .bytes, .void:
            return false
        default:
            return true
        }
    }

    private func rustUserTypeName(for type: ValdiModelPropertyType) -> String {
        switch type {
        case .nullable(let innerType):
            return "Optional<\(rustUserTypeName(for: innerType))>"
        case .string:
            return "String"
        case .double:
            return "Double"
        case .bool:
            return "Bool"
        case .long:
            return "Long"
        case .array(let elementType):
            return "Array<\(rustUserTypeName(for: elementType))>"
        case .bytes:
            return "Bytes"
        case .map(let keyType, let valueType):
            return "Map<\(rustUserTypeName(for: keyType)), \(rustUserTypeName(for: valueType))>"
        case .any:
            return "Any"
        case .void:
            return "()"
        case .function:
            return "Function"
        case .object(let mapping), .enum(let mapping):
            return rustTypeIdentifier(mapping.tsType)
        case .genericTypeParameter(let name):
            return rustTypeIdentifier(name)
        case .genericObject(let mapping, let typeArguments):
            let typeName = rustTypeIdentifier(mapping.tsType)
            if typeArguments.isEmpty {
                return typeName
            }
            return "\(typeName)<\(typeArguments.map { rustUserTypeName(for: $0) }.joined(separator: ", "))>"
        case .promise(let typeArgument):
            return "Promise<\(rustUserTypeName(for: typeArgument))>"
        }
    }

    private func rustUserArgumentExpression(type: ValdiModelPropertyType,
                                            name: String,
                                            callbackWrapperName: String? = nil) -> String {
        if case .function = type.unwrappingOptional, let callbackWrapperName {
            return "valdi_rust_module::\(callbackWrapperName)::from_handle(\(name))"
        }

        if case .promise = type.unwrappingOptional {
            return "valdi_rust::Promise::from_handle(\(name))"
        }

        if isRustHandleBackedUserType(type) {
            return "valdi_rust::ValdiRustTypedHandle::from_handle(\(name))"
        }

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
        if isPromiseType(returnType) {
            return "\(callExpression).into_ffi()"
        }

        if isRustHandleBackedUserType(returnType) {
            return "\(callExpression).into_return_handle()"
        }

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

    private func collectRustHandleTypeDefinitions(type: ValdiModelPropertyType,
                                                  definitions: inout Set<RustHandleTypeDefinition>) {
        switch type {
        case .nullable(let innerType):
            collectRustHandleTypeDefinitions(type: innerType, definitions: &definitions)
        case .array(let elementType):
            collectRustHandleTypeDefinitions(type: elementType, definitions: &definitions)
        case .map(let keyType, let valueType):
            collectRustHandleTypeDefinitions(type: keyType, definitions: &definitions)
            collectRustHandleTypeDefinitions(type: valueType, definitions: &definitions)
        case .function(let parameters, let returnType, _, _, _):
            parameters.forEach { collectRustHandleTypeDefinitions(type: $0.type, definitions: &definitions) }
            collectRustHandleTypeDefinitions(type: returnType, definitions: &definitions)
        case .object(let mapping), .enum(let mapping):
            let typeName = rustTypeIdentifier(mapping.tsType)
            if !typeName.isEmpty {
                definitions.insert(RustHandleTypeDefinition(name: typeName, typeParameterCount: 0))
            }
        case .genericObject(let mapping, let typeArguments):
            if isBridgeObservableMapping(mapping) {
                typeArguments.forEach { collectRustHandleTypeDefinitions(type: $0, definitions: &definitions) }
                return
            }

            let typeName = rustTypeIdentifier(mapping.tsType)
            if !typeName.isEmpty {
                definitions.insert(RustHandleTypeDefinition(name: typeName, typeParameterCount: typeArguments.count))
            }
            typeArguments.forEach { collectRustHandleTypeDefinitions(type: $0, definitions: &definitions) }
        case .promise(let typeArgument):
            collectRustHandleTypeDefinitions(type: typeArgument, definitions: &definitions)
        case .string, .double, .bool, .long, .bytes, .any, .void, .genericTypeParameter:
            return
        }
    }

    private func rustHandleTypeDefinitionsSource(definitions: Set<RustHandleTypeDefinition>) -> String {
        let sortedDefinitions = definitions.sorted {
            if $0.name == $1.name {
                return $0.typeParameterCount < $1.typeParameterCount
            }
            return $0.name < $1.name
        }

        return sortedDefinitions.map { definition in
            let markerName = "\(definition.name)HandleMarker"
            if definition.typeParameterCount == 0 {
                return """
                    pub enum \(markerName) {}
                    pub type \(definition.name) = crate::valdi_rust::ValdiRustTypedHandle<\(markerName)>;
                    """
            }

            let typeParameters = (0..<definition.typeParameterCount).map { "T\($0)" }
            let typeParameterList = typeParameters.joined(separator: ", ")
            let phantomType = typeParameters.count == 1 ? typeParameters[0] : "(\(typeParameterList))"
            return """
                pub struct \(markerName)<\(typeParameterList)>(std::marker::PhantomData<\(phantomType)>);
                pub type \(definition.name)<\(typeParameterList)> = crate::valdi_rust::ValdiRustTypedHandle<\(markerName)<\(typeParameterList)>>;
                """
        }.joined(separator: "\n\n")
    }

    private func rustFFIReturnType(for type: ValdiModelPropertyType) -> String? {
        switch type.unwrappingOptional {
        case .void:
            return nil
        case .promise:
            return "valdi_rust::ValdiRustPromise"
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
        case .promise:
            return FFIType(name: "ValdiRustPromise")
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

    private func callbackCallArgumentExpression(type: ValdiModelPropertyType,
                                                cppTypeName: String,
                                                name: String) -> (prelude: String, expression: String) {
        switch type.unwrappingOptional {
        case .double, .bool, .long:
            if type.isOptional {
                return ("", "valdiRustBridgeCopyBoxedValue<\(cppTypeName)>(\(name))")
            }
            return ("", name)
        case .string:
            if type.isOptional {
                return ("", "valdiRustBridgeCopyBoxedValue<\(cppTypeName)>(\(name))")
            }
            return ("", "valdiRustStringViewToStringBox(\(name))")
        case .bytes:
            if type.isOptional {
                return ("", "valdiRustBridgeCopyBoxedValue<\(cppTypeName)>(\(name))")
            }
            return ("", "valdiRustBytesViewToBytesView(\(name))")
        default:
            return ("", "valdiRustBridgeCopyBoxedValue<\(cppTypeName)>(\(name))")
        }
    }

    private func callbackReturnStatement(returnType: ValdiModelPropertyType,
                                         cppTypeName: String,
                                         callExpression: String) -> String {
        switch returnType.unwrappingOptional {
        case .void:
            return "\(callExpression);\n"
        case .double, .bool, .long:
            if returnType.isOptional {
                return "return valdiRustBridgeRetainBoxedValue<\(cppTypeName)>(\(callExpression));\n"
            }
            return "return \(callExpression);\n"
        case .string:
            if returnType.isOptional {
                return "return valdiRustBridgeRetainBoxedValue<\(cppTypeName)>(\(callExpression));\n"
            }
            return "return valdiRustStringBoxToOwnedString(\(callExpression));\n"
        case .bytes:
            if returnType.isOptional {
                return "return valdiRustBridgeRetainBoxedValue<\(cppTypeName)>(\(callExpression));\n"
            }
            return "return valdiRustBytesViewToOwnedBytes(\(callExpression));\n"
        default:
            return "return valdiRustBridgeRetainBoxedValue<\(cppTypeName)>(\(callExpression));\n"
        }
    }

    private func promiseResolverConfiguration(valueType: ValdiModelPropertyType,
                                              valueCppTypeName: String) -> String {
        if valueType.isOptional {
            return "resolver.resolveHandle = valdiRustPromiseResolveHandle<\(valueCppTypeName)>;"
        }

        switch valueType.unwrappingOptional {
        case .double:
            return "resolver.resolveDouble = valdiRustPromiseResolveDouble<\(valueCppTypeName)>;"
        case .bool:
            return "resolver.resolveBool = valdiRustPromiseResolveBool<\(valueCppTypeName)>;"
        case .long:
            return "resolver.resolveLong = valdiRustPromiseResolveLong<\(valueCppTypeName)>;"
        case .string:
            return "resolver.resolveString = valdiRustPromiseResolveString<\(valueCppTypeName)>;"
        case .bytes:
            return "resolver.resolveBytes = valdiRustPromiseResolveBytes<\(valueCppTypeName)>;"
        case .void:
            return "resolver.resolveVoid = valdiRustPromiseResolveVoid;"
        default:
            return "resolver.resolveHandle = valdiRustPromiseResolveHandle<\(valueCppTypeName)>;"
        }
    }

    private func promiseReturnStatement(returnType: ValdiModelPropertyType,
                                        valueCppTypeName: String,
                                        callExpression: String) -> String? {
        guard case .promise(let valueType) = returnType.unwrappingOptional else {
            return nil
        }

        let configuration = promiseResolverConfiguration(valueType: valueType, valueCppTypeName: valueCppTypeName)
        return """
            auto rustPromise = \(callExpression);
            return valdiRustMakePromiseFuture<\(valueCppTypeName)>(rustPromise, [](auto &resolver) {
                \(configuration)
            });

            """
    }

    private func rustCallbackCallArgument(type: ValdiModelPropertyType,
                                          name: String) -> (prelude: String, expression: String) {
        switch type.unwrappingOptional {
        case .string:
            if type.isOptional {
                return ("", "\(name).as_handle()")
            }
            let viewName = "\(name)_view"
            return ("let \(viewName) = crate::valdi_rust::ValdiRustStringView::from_str(&\(name));", viewName)
        case .bytes:
            if type.isOptional {
                return ("", "\(name).as_handle()")
            }
            let viewName = "\(name)_view"
            return ("let \(viewName) = crate::valdi_rust::ValdiRustBytesView::from_slice(&\(name));", viewName)
        case .double, .bool, .long:
            if type.isOptional {
                return ("", "\(name).as_handle()")
            }
            return ("", name)
        default:
            return ("", "\(name).as_handle()")
        }
    }

    private func rustCallbackReturnExpression(returnType: ValdiModelPropertyType,
                                              resultName: String) -> String {
        switch returnType.unwrappingOptional {
        case .void:
            return resultName
        case .string:
            if returnType.isOptional {
                return "crate::valdi_rust::ValdiRustTypedHandle::from_handle(\(resultName))"
            }
            return "\(resultName).into_string()"
        case .bytes:
            if returnType.isOptional {
                return "crate::valdi_rust::ValdiRustTypedHandle::from_handle(\(resultName))"
            }
            return "\(resultName).into_vec()"
        case .double, .bool, .long:
            if returnType.isOptional {
                return "crate::valdi_rust::ValdiRustTypedHandle::from_handle(\(resultName))"
            }
            return resultName
        default:
            return "crate::valdi_rust::ValdiRustTypedHandle::from_handle(\(resultName))"
        }
    }

    private func callbackThunkSource(definition: RustCallbackDefinition,
                                     callbackCppTypeName: String,
                                     parameterCppTypeNames: [String],
                                     returnCppTypeName: String) -> String {
        let ffiParameters = zip(definition.parameterTypes, definition.parameterNames).map {
            "\(ffiType(for: $0.0).name) \($0.1)"
        }.joined(separator: ", ")
        let parameterDeclarationSuffix = ffiParameters.isEmpty ? "" : ", \(ffiParameters)"
        let callbackArguments = zip(definition.parameterTypes, zip(definition.parameterNames, parameterCppTypeNames)).map {
            callbackCallArgumentExpression(type: $0.0, cppTypeName: $0.1.1, name: $0.1.0)
        }
        let prelude = callbackArguments.map { $0.prelude }
            .filter { !$0.isEmpty }
            .map { $0.indented }
            .joined(separator: "\n")
        let callArguments = callbackArguments.map { $0.expression }.joined(separator: ", ")
        let callExpression = "callbackBox->value(\(callArguments))"
        let body = callbackReturnStatement(returnType: definition.returnType,
                                           cppTypeName: returnCppTypeName,
                                           callExpression: callExpression)

        return """

            extern "C" \(ffiReturnType(for: definition.returnType).name) \(definition.symbolName)(ValdiRustHandle callback\(parameterDeclarationSuffix)) {
                auto *callbackBox = Valdi::unsafeBridgeUnretained<ValdiRustBridgeBox<\(callbackCppTypeName)>>(callback.ptr);
            \(prelude)
                \(body.indented)
            }

        """
    }

    private func rustCallbackWrapperSource(definition: RustCallbackDefinition) -> String {
        func moduleScopedABIType(_ type: String) -> String {
            return type.replacingOccurrences(of: "valdi_rust::", with: "crate::valdi_rust::")
        }

        let markerName = "\(definition.wrapperName)HandleMarker"
        let parameterDeclarations = zip(definition.parameterTypes, definition.parameterNames).map {
            "\($0.1): \(rustUserTypeName(for: $0.0))"
        }.joined(separator: ", ")
        let externParameters = zip(definition.parameterTypes, definition.parameterNames).map {
            "\($0.1): \(moduleScopedABIType(rustFFIParameterType(for: $0.0)))"
        }.joined(separator: ", ")
        let externParameterSuffix = externParameters.isEmpty ? "" : ", \(externParameters)"
        let callArguments = zip(definition.parameterTypes, definition.parameterNames).map {
            rustCallbackCallArgument(type: $0.0, name: $0.1)
        }
        let prelude = callArguments.map { $0.prelude }
            .filter { !$0.isEmpty }
            .map { "        \($0)" }
            .joined(separator: "\n")
        let callArgumentSuffix = callArguments.map { $0.expression }.isEmpty ? "" : ", \(callArguments.map { $0.expression }.joined(separator: ", "))"
        let returnTypeName = rustUserTypeName(for: definition.returnType)
        let rustReturnDeclaration = definition.returnType.unwrappingOptional.isVoid ? "" : " -> \(returnTypeName)"
        let externReturnDeclaration = rustFFIReturnType(for: definition.returnType).map { " -> \(moduleScopedABIType($0))" } ?? ""
        let methodParameterList = parameterDeclarations.isEmpty ? "&self" : "&self, \(parameterDeclarations)"
        let callBody: String
        if definition.returnType.unwrappingOptional.isVoid {
            callBody = "unsafe { \(definition.symbolName)(self.0.as_handle()\(callArgumentSuffix)); }"
        } else {
            callBody = """
                    let result = unsafe { \(definition.symbolName)(self.0.as_handle()\(callArgumentSuffix)) };
                    \(rustCallbackReturnExpression(returnType: definition.returnType, resultName: "result"))
            """
        }
        let preludeBlock = prelude.isEmpty ? "" : "\(prelude)\n"

        return """

            pub enum \(markerName) {}

            #[derive(Clone, Copy)]
            pub struct \(definition.wrapperName)(crate::valdi_rust::ValdiRustTypedHandle<\(markerName)>);

            unsafe extern "C" {
                fn \(definition.symbolName)(callback: crate::valdi_rust::ValdiRustHandle\(externParameterSuffix))\(externReturnDeclaration);
            }

            impl \(definition.wrapperName) {
                pub fn from_handle(handle: crate::valdi_rust::ValdiRustHandle) -> Self {
                    Self(crate::valdi_rust::ValdiRustTypedHandle::from_handle(handle))
                }

                pub fn as_handle(&self) -> crate::valdi_rust::ValdiRustHandle {
                    self.0.as_handle()
                }

                pub fn retain_for_storage(&self) -> crate::valdi_rust::ValdiRustRetainedHandle<\(markerName)> {
                    self.0.retain_for_storage()
                }

                pub fn from_retained_handle(handle: &crate::valdi_rust::ValdiRustRetainedHandle<\(markerName)>) -> Self {
                    Self(handle.as_typed_handle())
                }

                pub fn call(\(methodParameterList))\(rustReturnDeclaration) {
            \(preludeBlock)\(callBody)
                }
            }

        """
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

            struct ValdiRustPromiseResolver {
                void *context;
                void (*retain)(void *context);
                void (*release)(void *context);
                void (*resolveDouble)(void *context, double value);
                void (*resolveBool)(void *context, bool value);
                void (*resolveLong)(void *context, int64_t value);
                void (*resolveString)(void *context, ValdiRustStringView value);
                void (*resolveBytes)(void *context, ValdiRustBytesView value);
                void (*resolveHandle)(void *context, ValdiRustHandle value);
                void (*resolveVoid)(void *context);
                void (*reject)(void *context, ValdiRustStringView message);
            };

            struct ValdiRustPromise {
                void *context;
                void (*run)(void *context, ValdiRustPromiseResolver resolver);
                void (*release)(void *context);
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

            [[maybe_unused]] static void valdiRustCppFreeOwnedString(const char *data, size_t /*len*/) {
                delete[] data;
            }

            [[maybe_unused]] static void valdiRustCppFreeOwnedBytes(const uint8_t *data, size_t /*len*/) {
                delete[] data;
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

            [[maybe_unused]] static ValdiRustOwnedString valdiRustStringBoxToOwnedString(const Valdi::StringBox &string) {
                auto view = string.toStringView();
                char *data = nullptr;
                if (!view.empty()) {
                    data = new char[view.size()];
                    std::memcpy(data, view.data(), view.size());
                }
                return ValdiRustOwnedString{data, view.size(), valdiRustCppFreeOwnedString};
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

            [[maybe_unused]] static ValdiRustOwnedBytes valdiRustBytesViewToOwnedBytes(const Valdi::BytesView &bytes) {
                uint8_t *data = nullptr;
                if (!bytes.empty()) {
                    data = new uint8_t[bytes.size()];
                    std::memcpy(data, bytes.data(), bytes.size());
                }
                return ValdiRustOwnedBytes{data, bytes.size(), valdiRustCppFreeOwnedBytes};
            }

            template<typename T>
            [[maybe_unused]] static T valdiRustBridgeTakeBoxedValue(ValdiRustHandle handle) {
                auto box = Valdi::unsafeBridgeTransfer<ValdiRustBridgeBox<T>>(handle.ptr);
                return std::move(box->value);
            }

            template<typename T>
            [[maybe_unused]] static T valdiRustBridgeCopyBoxedValue(ValdiRustHandle handle) {
                auto *box = Valdi::unsafeBridgeUnretained<ValdiRustBridgeBox<T>>(handle.ptr);
                return box->value;
            }

            template<typename T, typename U>
            [[maybe_unused]] static ValdiRustHandle valdiRustBridgeRetainBoxedValue(U &&value) {
                auto valueBox = Valdi::makeShared<ValdiRustBridgeBox<T>>(std::forward<U>(value));
                return ValdiRustHandle{Valdi::unsafeBridgeRetain(valueBox.get()), valdiRustBridgeRetain, valdiRustBridgeRelease};
            }

            template<typename T>
            class ValdiRustPromiseResolverBox final : public Valdi::SimpleRefCountable {
            public:
                Valdi::TypedPromise<T> promise;
            };

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseSetValue(void *context, const T &value) {
                auto *box = Valdi::unsafeBridgeUnretained<ValdiRustPromiseResolverBox<T>>(context);
                box->promise.setValue(value);
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseReject(void *context, ValdiRustStringView message) {
                auto *box = Valdi::unsafeBridgeUnretained<ValdiRustPromiseResolverBox<T>>(context);
                box->promise.setError(Valdi::Error(valdiRustStringViewToStringBox(message)));
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseResolveDouble(void *context, double value) {
                valdiRustPromiseSetValue<T>(context, value);
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseResolveBool(void *context, bool value) {
                valdiRustPromiseSetValue<T>(context, value);
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseResolveLong(void *context, int64_t value) {
                valdiRustPromiseSetValue<T>(context, value);
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseResolveString(void *context, ValdiRustStringView value) {
                valdiRustPromiseSetValue<T>(context, valdiRustStringViewToStringBox(value));
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseResolveBytes(void *context, ValdiRustBytesView value) {
                valdiRustPromiseSetValue<T>(context, valdiRustBytesViewToBytesView(value));
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseResolveHandle(void *context, ValdiRustHandle value) {
                valdiRustPromiseSetValue<T>(context, valdiRustBridgeTakeBoxedValue<T>(value));
            }

            [[maybe_unused]] static void valdiRustPromiseResolveVoid(void *context) {
                auto *box = Valdi::unsafeBridgeUnretained<ValdiRustPromiseResolverBox<void>>(context);
                box->promise.setValue();
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseRejectWrongResolver(void *context, const char *message) {
                auto view = ValdiRustStringView{message, std::strlen(message)};
                valdiRustPromiseReject<T>(context, view);
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseResolveDoubleMismatch(void *context, double /*value*/) {
                valdiRustPromiseRejectWrongResolver<T>(context, "Rust promise resolved with an unexpected double value");
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseResolveBoolMismatch(void *context, bool /*value*/) {
                valdiRustPromiseRejectWrongResolver<T>(context, "Rust promise resolved with an unexpected bool value");
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseResolveLongMismatch(void *context, int64_t /*value*/) {
                valdiRustPromiseRejectWrongResolver<T>(context, "Rust promise resolved with an unexpected long value");
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseResolveStringMismatch(void *context, ValdiRustStringView /*value*/) {
                valdiRustPromiseRejectWrongResolver<T>(context, "Rust promise resolved with an unexpected string value");
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseResolveBytesMismatch(void *context, ValdiRustBytesView /*value*/) {
                valdiRustPromiseRejectWrongResolver<T>(context, "Rust promise resolved with an unexpected bytes value");
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseResolveHandleMismatch(void *context, ValdiRustHandle /*value*/) {
                valdiRustPromiseRejectWrongResolver<T>(context, "Rust promise resolved with an unexpected handle value");
            }

            template<typename T>
            [[maybe_unused]] static void valdiRustPromiseResolveVoidMismatch(void *context) {
                valdiRustPromiseRejectWrongResolver<T>(context, "Rust promise resolved with an unexpected void value");
            }

            template<typename T, typename ConfigureResolver>
            [[maybe_unused]] static Valdi::Future<T> valdiRustMakePromiseFuture(ValdiRustPromise rustPromise,
                                                                                ConfigureResolver configureResolver) {
                auto box = Valdi::makeShared<ValdiRustPromiseResolverBox<T>>();
                auto future = box->promise.getFuture();
                ValdiRustPromiseResolver resolver{
                    Valdi::unsafeBridgeRetain(box.get()),
                    valdiRustBridgeRetain,
                    valdiRustBridgeRelease,
                    valdiRustPromiseResolveDoubleMismatch<T>,
                    valdiRustPromiseResolveBoolMismatch<T>,
                    valdiRustPromiseResolveLongMismatch<T>,
                    valdiRustPromiseResolveStringMismatch<T>,
                    valdiRustPromiseResolveBytesMismatch<T>,
                    valdiRustPromiseResolveHandleMismatch<T>,
                    valdiRustPromiseResolveVoidMismatch<T>,
                    valdiRustPromiseReject<T>
                };
                configureResolver(resolver);

                if (rustPromise.run == nullptr) {
                    box->promise.setError(Valdi::Error("Rust promise is missing an executor"));
                    resolver.release(resolver.context);
                    if (rustPromise.release != nullptr) {
                        rustPromise.release(rustPromise.context);
                    }
                    return future;
                }

                rustPromise.run(rustPromise.context, resolver);
                return future;
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
        generator.includeSection.addInclude(path: "valdi_core/cpp/Utils/Future.hpp")
        generator.includeSection.addInclude(path: "valdi_core/cpp/Utils/StringBox.hpp")
        generator.includeSection.addSystemInclude(path: "cstring")
        generator.includeSection.addSystemInclude(path: "cstdint")
        generator.includeSection.addSystemInclude(path: "cstddef")
        generator.includeSection.addSystemInclude(path: "optional")
        generator.includeSection.addSystemInclude(path: "string_view")
        generator.includeSection.addSystemInclude(path: "type_traits")
        generator.includeSection.addSystemInclude(path: "utility")

        generator.body.appendBody(FileHeaderCommentGenerator.generateComment(sourceFilename: sourceFileName, additionalComments: """
            Generated Rust bridge for \(cppType.declaration.fullTypeName).
            Rust symbols are derived from the @ExportModule TypeScript declaration.

            """))

        writeCommonRuntime(to: generator.body)

        var methodImplementations = ""
        var externDeclarations = ""
        var rustAdapterFunctions = ""
        var callbackThunks = ""
        var rustCallbackWrappers = ""
        var rustHandleTypeDefinitions = Set<RustHandleTypeDefinition>()

        for property in exportedModule.model.properties {
            switch property.type {
            case .function(let parameters, let returnType, _, _, _):
                parameters.forEach { collectRustHandleTypeDefinitions(type: $0.type, definitions: &rustHandleTypeDefinitions) }
                collectRustHandleTypeDefinitions(type: returnType, definitions: &rustHandleTypeDefinitions)

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
                let callbackWrapperNames: [String?] = try parameters.enumerated().map { index, parameter in
                    guard case .function(let callbackParameters, let callbackReturnType, _, _, _) = parameter.type.unwrappingOptional else {
                        return nil
                    }

                    let wrapperName = rustCallbackWrapperName(methodName: propertyName.methodName, parameterName: parameter.name)
                    let symbolName = rustCallbackSymbolName(methodName: propertyName.methodName, parameterName: parameter.name)
                    let callbackParameterNames = callbackParameters.indices.map { "arg\($0)" }
                    let definition = RustCallbackDefinition(wrapperName: wrapperName,
                                                            symbolName: symbolName,
                                                            parameterNames: callbackParameterNames,
                                                            parameterTypes: callbackParameters.map { $0.type },
                                                            returnType: callbackReturnType)
                    let callbackParameterCppTypeNames = try callbackParameters.enumerated().map { callbackIndex, callbackParameter in
                        try typeGenerator.getTypeParser(type: callbackParameter.type,
                                                        namePaths: [property.name, parameter.name, "Argument\(callbackIndex)"],
                                                        nameAllocator: nameAllocator.scoped()).typeNameResolver.resolve(cppType.declaration.namespace)
                    }
                    let callbackReturnCppTypeName = try typeGenerator.getTypeParser(type: callbackReturnType,
                                                                                    namePaths: [property.name, parameter.name, "Return"],
                                                                                    nameAllocator: nameAllocator.scoped()).typeNameResolver.resolve(cppType.declaration.namespace)
                    let callbackCppTypeName = cppMethod.parameters[index].typeNameResolver.resolve(cppType.declaration.namespace)
                    callbackThunks += callbackThunkSource(definition: definition,
                                                          callbackCppTypeName: callbackCppTypeName,
                                                          parameterCppTypeNames: callbackParameterCppTypeNames,
                                                          returnCppTypeName: callbackReturnCppTypeName)
                    rustCallbackWrappers += rustCallbackWrapperSource(definition: definition)
                    return wrapperName
                }
                let rustUserArguments = parameters.indices.map {
                    rustUserArgumentExpression(type: parameters[$0].type,
                                               name: rustParameterNames[$0],
                                               callbackWrapperName: callbackWrapperNames[$0])
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
                let cppReturnBody: String
                if case .promise(let promiseValueType) = returnType.unwrappingOptional {
                    let promiseValueTypeParser = try typeGenerator.getTypeParser(type: promiseValueType,
                                                                                  namePaths: [property.name, "PromiseValue"],
                                                                                  nameAllocator: nameAllocator.scoped())
                    let promiseValueCppType = promiseValueTypeParser.typeNameResolver.resolve(cppType.declaration.namespace)
                    cppReturnBody = promiseReturnStatement(returnType: returnType,
                                                           valueCppTypeName: promiseValueCppType,
                                                           callExpression: "\(rustSymbol)(\(rustCallArguments))")!
                } else {
                    cppReturnBody = returnStatement(returnType: returnType,
                                                    cppTypeName: returnCppType,
                                                    callExpression: "\(rustSymbol)(\(rustCallArguments))")
                }
                methodImplementations += """

                    \(returnTypeName) \(propertyName.methodName)(\(cppParameters.joined(separator: ", "))) final {
                \(ffiParameters.map { $0.prelude }.filter { !$0.isEmpty }.map { $0.indented }.joined(separator: "\n"))
                        \(cppReturnBody.indented)
                    }

                """
            default:
                collectRustHandleTypeDefinitions(type: property.type, definitions: &rustHandleTypeDefinitions)

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
                let cppReturnBody: String
                if case .promise(let promiseValueType) = property.type.unwrappingOptional {
                    let promiseValueTypeParser = try typeGenerator.getTypeParser(type: promiseValueType,
                                                                                  namePaths: [property.name, "PromiseValue"],
                                                                                  nameAllocator: nameAllocator.scoped())
                    let promiseValueCppType = promiseValueTypeParser.typeNameResolver.resolve(cppType.declaration.namespace)
                    cppReturnBody = promiseReturnStatement(returnType: property.type,
                                                           valueCppTypeName: promiseValueCppType,
                                                           callExpression: "\(rustSymbol)()")!
                } else {
                    cppReturnBody = returnStatement(returnType: property.type,
                                                    cppTypeName: returnCppType,
                                                    callExpression: "\(rustSymbol)()")
                }
                methodImplementations += """

                    \(returnCppType) \(propertyName.methodName)() final {
                        \(cppReturnBody.indented)
                    }

                """
            }
        }

        generator.body.appendBody("""
            \(callbackThunks)

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
                         file: .data(try rustAdapterSource(typeDefinitions: rustHandleTypeDefinitionsSource(definitions: rustHandleTypeDefinitions),
                                                           callbackWrappers: rustCallbackWrappers,
                                                           functions: rustAdapterFunctions).utf8Data()),
                         groupingIdentifier: "\(bundleInfo.name).rust_bridge.rs",
                         groupingPriority: 0)
        ]
    }

    private func rustAdapterSource(typeDefinitions: String, callbackWrappers: String, functions: String) -> String {
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

            \(typeDefinitions)
            \(callbackWrappers)

                include!(env!("VALDI_RUST_USER_ROOT_PATH"));
            }
            \(functions)
            """.indented
    }
}
