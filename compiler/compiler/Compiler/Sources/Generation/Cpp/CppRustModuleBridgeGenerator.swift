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
    private let generatedModelsByTypeName: [String: ValdiModel]
    private let generatedEnumsByTypeName: [String: ExportedEnum]

    init(bundleInfo: CompilationItem.BundleInfo,
         cppType: CPPType,
         exportedModule: ExportedModule,
         sourceFileName: GeneratedSourceFilename,
         generatedModelsByTypeName: [String: ValdiModel] = [:],
         generatedEnumsByTypeName: [String: ExportedEnum] = [:]) {
        self.bundleInfo = bundleInfo
        self.cppType = cppType
        self.exportedModule = exportedModule
        self.sourceFileName = sourceFileName
        self.generatedModelsByTypeName = generatedModelsByTypeName
        self.generatedEnumsByTypeName = generatedEnumsByTypeName
    }

    private struct CppPropertyName {
        let methodName: String
    }

    private struct CppModelPropertyName {
        let getterName: String
        let setterName: String
        let constructorParameterName: String
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

    private struct RustModelDefinition {
        let rustName: String
        let cppTypeName: String
        let model: ValdiModel
    }

    private struct RustEnumDefinition {
        let rustName: String
        let exportedEnum: ExportedEnum
    }

    private enum RustBridgeObservableKind: Equatable {
        case double
        case bool
        case long
        case string
        case bytes
        case handle
        case enumValue

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
            case .enumValue:
                return "ValdiRustLongObservableObserver"
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
            case .enumValue:
                return "valdiRustMakeEnumBridgeObservable"
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

    private func resolveModelPropertyNames(model: ValdiModel) -> [String: CppModelPropertyName] {
        let nameAllocator = PropertyNameAllocator.forCpp()
        if let cppType = model.cppType {
            _ = nameAllocator.allocate(property: "\(cppType.declaration.name)Proxy")
        }
        ["getRegisteredClass", "registeredClass"].forEach {
            _ = nameAllocator.allocate(property: $0)
        }

        var namesByProperty = [String: CppModelPropertyName]()
        for property in model.properties {
            _ = nameAllocator.allocate(property: "_\(property.name)")
            let getterName = nameAllocator.allocate(property: "get\(property.name.pascalCased)").name
            let setterName = nameAllocator.allocate(property: "set\(property.name.pascalCased)").name
            let constructorParameterName = nameAllocator.allocate(property: property.name).name
            namesByProperty[property.name] = CppModelPropertyName(getterName: getterName,
                                                                  setterName: setterName,
                                                                  constructorParameterName: constructorParameterName)
        }

        return namesByProperty
    }

    private func generatedModel(for mapping: ValdiNodeClassMapping) -> ValdiModel? {
        guard mapping.isGenerated else {
            return nil
        }

        if let fullTypeName = mapping.cppType?.declaration.fullTypeName,
           let model = generatedModelsByTypeName[fullTypeName] {
            return model
        }

        if let typeName = mapping.cppType?.declaration.name,
           let model = generatedModelsByTypeName[typeName] {
            return model
        }

        return generatedModelsByTypeName[mapping.tsType]
    }

    private func generatedEnum(for mapping: ValdiNodeClassMapping) -> ExportedEnum? {
        guard mapping.isGenerated else {
            return nil
        }

        if let fullTypeName = mapping.cppType?.declaration.fullTypeName,
           let exportedEnum = generatedEnumsByTypeName[fullTypeName] {
            return exportedEnum
        }

        if let typeName = mapping.cppType?.declaration.name,
           let exportedEnum = generatedEnumsByTypeName[typeName] {
            return exportedEnum
        }

        return generatedEnumsByTypeName[mapping.tsType]
    }

    private func isGeneratedRustEnumType(_ type: ValdiModelPropertyType) -> Bool {
        guard !type.isOptional,
              case .enum(let mapping) = type.unwrappingOptional else {
            return false
        }

        return generatedEnum(for: mapping)?.cppType != nil
    }

    private func isLocalRustWrapperType(_ type: ValdiModelPropertyType) -> Bool {
        switch type.unwrappingOptional {
        case .object, .genericObject:
            return !type.isOptional
        case .enum:
            return !type.isOptional && !isGeneratedRustEnumType(type)
        default:
            return false
        }
    }

    private func collectRustGeneratedModelDefinitions(type: ValdiModelPropertyType,
                                                      definitions: inout [String: RustModelDefinition]) {
        switch type {
        case .nullable(let innerType):
            collectRustGeneratedModelDefinitions(type: innerType, definitions: &definitions)
        case .array(let elementType):
            collectRustGeneratedModelDefinitions(type: elementType, definitions: &definitions)
        case .map(let keyType, let valueType):
            collectRustGeneratedModelDefinitions(type: keyType, definitions: &definitions)
            collectRustGeneratedModelDefinitions(type: valueType, definitions: &definitions)
        case .function(let parameters, let returnType, _, _, _):
            parameters.forEach { collectRustGeneratedModelDefinitions(type: $0.type, definitions: &definitions) }
            collectRustGeneratedModelDefinitions(type: returnType, definitions: &definitions)
        case .object(let mapping), .enum(let mapping):
            guard let model = generatedModel(for: mapping),
                  !model.exportAsInterface,
                  model.typeParameters == nil,
                  let modelCppType = model.cppType else {
                return
            }

            let key = modelCppType.declaration.fullTypeName
            if definitions[key] == nil {
                definitions[key] = RustModelDefinition(rustName: rustTypeIdentifier(mapping.tsType),
                                                       cppTypeName: modelCppType.declaration.resolveTypeName(inNamespace: cppType.declaration.namespace),
                                                       model: model)
                model.properties.forEach {
                    collectRustGeneratedModelDefinitions(type: $0.type, definitions: &definitions)
                }
            }
        case .genericObject(let mapping, let typeArguments):
            typeArguments.forEach { collectRustGeneratedModelDefinitions(type: $0, definitions: &definitions) }
            guard let model = generatedModel(for: mapping),
                  !model.exportAsInterface,
                  model.typeParameters == nil,
                  typeArguments.isEmpty,
                  let modelCppType = model.cppType else {
                return
            }

            let key = modelCppType.declaration.fullTypeName
            if definitions[key] == nil {
                definitions[key] = RustModelDefinition(rustName: rustTypeIdentifier(mapping.tsType),
                                                       cppTypeName: modelCppType.declaration.resolveTypeName(inNamespace: cppType.declaration.namespace),
                                                       model: model)
                model.properties.forEach {
                    collectRustGeneratedModelDefinitions(type: $0.type, definitions: &definitions)
                }
            }
        case .promise(let typeArgument):
            collectRustGeneratedModelDefinitions(type: typeArgument, definitions: &definitions)
        case .string, .double, .bool, .long, .bytes, .any, .void, .genericTypeParameter:
            return
        }
    }

    private func collectRustGeneratedEnumDefinitions(type: ValdiModelPropertyType,
                                                     definitions: inout [String: RustEnumDefinition]) {
        var visitedModelTypes = Set<String>()
        collectRustGeneratedEnumDefinitions(type: type,
                                            definitions: &definitions,
                                            visitedModelTypes: &visitedModelTypes)
    }

    private func collectRustGeneratedEnumDefinitions(type: ValdiModelPropertyType,
                                                     definitions: inout [String: RustEnumDefinition],
                                                     visitedModelTypes: inout Set<String>) {
        switch type {
        case .nullable(let innerType):
            collectRustGeneratedEnumDefinitions(type: innerType,
                                                definitions: &definitions,
                                                visitedModelTypes: &visitedModelTypes)
        case .array(let elementType):
            collectRustGeneratedEnumDefinitions(type: elementType,
                                                definitions: &definitions,
                                                visitedModelTypes: &visitedModelTypes)
        case .map(let keyType, let valueType):
            collectRustGeneratedEnumDefinitions(type: keyType,
                                                definitions: &definitions,
                                                visitedModelTypes: &visitedModelTypes)
            collectRustGeneratedEnumDefinitions(type: valueType,
                                                definitions: &definitions,
                                                visitedModelTypes: &visitedModelTypes)
        case .function(let parameters, let returnType, _, _, _):
            parameters.forEach {
                collectRustGeneratedEnumDefinitions(type: $0.type,
                                                    definitions: &definitions,
                                                    visitedModelTypes: &visitedModelTypes)
            }
            collectRustGeneratedEnumDefinitions(type: returnType,
                                                definitions: &definitions,
                                                visitedModelTypes: &visitedModelTypes)
        case .enum(let mapping):
            guard let exportedEnum = generatedEnum(for: mapping),
                  let enumCppType = exportedEnum.cppType else {
                return
            }

            let key = enumCppType.declaration.fullTypeName
            if definitions[key] == nil {
                definitions[key] = RustEnumDefinition(rustName: rustTypeIdentifier(mapping.tsType),
                                                      exportedEnum: exportedEnum)
            }
        case .object(let mapping):
            guard let model = generatedModel(for: mapping),
                  let modelCppType = model.cppType else {
                return
            }

            let key = modelCppType.declaration.fullTypeName
            guard visitedModelTypes.insert(key).inserted else {
                return
            }

            model.properties.forEach {
                collectRustGeneratedEnumDefinitions(type: $0.type,
                                                    definitions: &definitions,
                                                    visitedModelTypes: &visitedModelTypes)
            }
        case .genericObject(let mapping, let typeArguments):
            typeArguments.forEach {
                collectRustGeneratedEnumDefinitions(type: $0,
                                                    definitions: &definitions,
                                                    visitedModelTypes: &visitedModelTypes)
            }

            guard let model = generatedModel(for: mapping),
                  typeArguments.isEmpty,
                  let modelCppType = model.cppType else {
                return
            }

            let key = modelCppType.declaration.fullTypeName
            guard visitedModelTypes.insert(key).inserted else {
                return
            }

            model.properties.forEach {
                collectRustGeneratedEnumDefinitions(type: $0.type,
                                                    definitions: &definitions,
                                                    visitedModelTypes: &visitedModelTypes)
            }
        case .promise(let typeArgument):
            collectRustGeneratedEnumDefinitions(type: typeArgument,
                                                definitions: &definitions,
                                                visitedModelTypes: &visitedModelTypes)
        case .string, .double, .bool, .long, .bytes, .any, .void, .genericTypeParameter:
            return
        }
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
            case .enum(_) where isGeneratedRustEnumType(valueType):
                return .enumValue
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

        if isGeneratedRustEnumType(type) {
            return "i64"
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

        if isGeneratedRustEnumType(type) {
            return false
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

        if isGeneratedRustEnumType(type) {
            return "valdi_rust_module::\(rustUserTypeName(for: type))::from_ffi(\(name))"
        }

        if isRustHandleBackedUserType(type) {
            if isLocalRustWrapperType(type) {
                return "valdi_rust_module::\(rustUserTypeName(for: type))::from_handle(\(name))"
            }
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

        if isGeneratedRustEnumType(returnType) {
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
        case .object(let mapping):
            let typeName = rustTypeIdentifier(mapping.tsType)
            if !typeName.isEmpty {
                definitions.insert(RustHandleTypeDefinition(name: typeName, typeParameterCount: 0))
            }
        case .enum(let mapping):
            if generatedEnum(for: mapping) != nil {
                return
            }

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

                    pub struct \(definition.name)(crate::valdi_rust::ValdiRustRetainedHandle<\(markerName)>);

                    unsafe impl Send for \(definition.name) {}

                    impl Clone for \(definition.name) {
                        fn clone(&self) -> Self {
                            Self(self.0.clone())
                        }
                    }

                    impl \(definition.name) {
                        pub fn from_handle(handle: crate::valdi_rust::ValdiRustHandle) -> Self {
                            let typed = crate::valdi_rust::ValdiRustTypedHandle::<\(markerName)>::from_handle(handle);
                            Self(typed.retain_for_storage())
                        }

                        pub fn from_owned_handle(handle: crate::valdi_rust::ValdiRustHandle) -> Self {
                            Self(crate::valdi_rust::ValdiRustRetainedHandle::from_owned_handle(handle))
                        }

                        pub fn from_retained_handle(handle: &crate::valdi_rust::ValdiRustRetainedHandle<\(markerName)>) -> Self {
                            Self(handle.clone())
                        }

                        pub fn as_handle(&self) -> crate::valdi_rust::ValdiRustHandle {
                            self.0.as_handle()
                        }

                        pub fn retain_for_storage(&self) -> crate::valdi_rust::ValdiRustRetainedHandle<\(markerName)> {
                            self.0.clone()
                        }

                        pub fn into_return_handle(self) -> crate::valdi_rust::ValdiRustHandle {
                            let handle = self.0.as_handle();
                            std::mem::forget(self);
                            handle
                        }
                    }

                    impl crate::valdi_rust::ValdiRustPromiseValue for \(definition.name) {
                        fn resolve_with(resolver: &crate::valdi_rust::ValdiRustPromiseResolver, value: Self) {
                            (resolver.resolve_handle)(resolver.context, value.into_return_handle());
                        }
                    }

                    impl crate::valdi_rust::ValdiRustObservableValue for \(definition.name) {
                        type Observer = crate::valdi_rust::ValdiRustHandleObservableObserver;
                    }

                    impl crate::valdi_rust::ValdiRustObserver<\(definition.name)> for crate::valdi_rust::ValdiRustHandleObservableObserver {
                        fn next(&self, value: \(definition.name)) {
                            crate::valdi_rust::ValdiRustHandleObservableObserver::next(self, value.into_return_handle());
                        }

                        fn release(&self) {
                            crate::valdi_rust::ValdiRustHandleObservableObserver::release(self);
                        }
                    }
                    """
            }

            let typeParameters = (0..<definition.typeParameterCount).map { "T\($0)" }
            let typeParameterList = typeParameters.joined(separator: ", ")
            let typeParameterBounds = typeParameters.map { "\($0): Send + 'static" }.joined(separator: ", ")
            let phantomType = typeParameters.count == 1 ? typeParameters[0] : "(\(typeParameterList))"
            return """
                pub struct \(markerName)<\(typeParameterList)>(std::marker::PhantomData<\(phantomType)>);

                pub struct \(definition.name)<\(typeParameterList)>(crate::valdi_rust::ValdiRustRetainedHandle<\(markerName)<\(typeParameterList)>>);

                unsafe impl<\(typeParameterList)> Send for \(definition.name)<\(typeParameterList)> {}

                impl<\(typeParameterList)> Clone for \(definition.name)<\(typeParameterList)> {
                    fn clone(&self) -> Self {
                        Self(self.0.clone())
                    }
                }

                impl<\(typeParameterList)> \(definition.name)<\(typeParameterList)> {
                    pub fn from_handle(handle: crate::valdi_rust::ValdiRustHandle) -> Self {
                        let typed = crate::valdi_rust::ValdiRustTypedHandle::<\(markerName)<\(typeParameterList)>>::from_handle(handle);
                        Self(typed.retain_for_storage())
                    }

                    pub fn from_owned_handle(handle: crate::valdi_rust::ValdiRustHandle) -> Self {
                        Self(crate::valdi_rust::ValdiRustRetainedHandle::from_owned_handle(handle))
                    }

                    pub fn from_retained_handle(handle: &crate::valdi_rust::ValdiRustRetainedHandle<\(markerName)<\(typeParameterList)>>) -> Self {
                        Self(handle.clone())
                    }

                    pub fn as_handle(&self) -> crate::valdi_rust::ValdiRustHandle {
                        self.0.as_handle()
                    }

                    pub fn retain_for_storage(&self) -> crate::valdi_rust::ValdiRustRetainedHandle<\(markerName)<\(typeParameterList)>> {
                        self.0.clone()
                    }

                    pub fn into_return_handle(self) -> crate::valdi_rust::ValdiRustHandle {
                        let handle = self.0.as_handle();
                        std::mem::forget(self);
                        handle
                    }
                }

                impl<\(typeParameterBounds)> crate::valdi_rust::ValdiRustPromiseValue for \(definition.name)<\(typeParameterList)> {
                    fn resolve_with(resolver: &crate::valdi_rust::ValdiRustPromiseResolver, value: Self) {
                        (resolver.resolve_handle)(resolver.context, value.into_return_handle());
                    }
                }

                impl<\(typeParameterBounds)> crate::valdi_rust::ValdiRustObservableValue for \(definition.name)<\(typeParameterList)> {
                    type Observer = crate::valdi_rust::ValdiRustHandleObservableObserver;
                }

                impl<\(typeParameterBounds)> crate::valdi_rust::ValdiRustObserver<\(definition.name)<\(typeParameterList)>> for crate::valdi_rust::ValdiRustHandleObservableObserver {
                    fn next(&self, value: \(definition.name)<\(typeParameterList)>) {
                        crate::valdi_rust::ValdiRustHandleObservableObserver::next(self, value.into_return_handle());
                    }

                    fn release(&self) {
                        crate::valdi_rust::ValdiRustHandleObservableObserver::release(self);
                    }
                }
                """
        }.joined(separator: "\n\n")
    }

    private func rustEnumHelperRustSource(definition: RustEnumDefinition) -> String {
        let caseData: [(name: String, ordinal: Int, valueLiteral: String, valueType: String, comments: String?)]
        switch definition.exportedEnum.cases {
        case .enum(let intCases):
            caseData = intCases.enumerated().map { index, enumCase in
                (name: enumCase.name,
                 ordinal: index,
                 valueLiteral: "\(enumCase.value)",
                 valueType: "i64",
                 comments: enumCase.comments)
            }
        case .stringEnum(let stringCases):
            caseData = stringCases.enumerated().map { index, enumCase in
                (name: enumCase.name,
                 ordinal: index,
                 valueLiteral: "\"\(enumCase.value.jsonEscaped)\"",
                 valueType: "&'static str",
                 comments: enumCase.comments)
            }
        }

        var usedVariantNames = Set<String>()
        let variants = caseData.map { enumCase -> (name: String, ordinal: Int, valueLiteral: String, comments: String?) in
            let baseName = rustTypeIdentifier(enumCase.name)
            var variantName = baseName
            var index = 2
            while usedVariantNames.contains(variantName) {
                variantName = "\(baseName)\(index)"
                index += 1
            }
            usedVariantNames.insert(variantName)
            return (name: variantName, ordinal: enumCase.ordinal, valueLiteral: enumCase.valueLiteral, comments: enumCase.comments)
        }
        let valueType = caseData.first?.valueType ?? "i64"
        let variantDeclarations = variants.map { variant -> String in
            let comments = variant.comments.map {
                "\(FileHeaderCommentGenerator.generateMultilineComment(comment: $0))\n"
            } ?? ""
            return "\(comments)\(variant.name) = \(variant.ordinal),"
        }.joined(separator: "\n")
        let fromFFIArms = variants.map {
            "\($0.ordinal) => Self::\($0.name),"
        }.joined(separator: "\n")
        let valueArms = variants.map {
            "Self::\($0.name) => \($0.valueLiteral),"
        }.joined(separator: "\n")

        return """

            #[repr(i64)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
            pub enum \(definition.rustName) {
            \(variantDeclarations.indented)
            }

            impl \(definition.rustName) {
                pub fn from_ffi(value: i64) -> Self {
                    match value {
            \(fromFFIArms.indented.indented.indented)
                        _ => panic!("Invalid \(definition.rustName) ordinal {}", value),
                    }
                }

                pub fn into_ffi(self) -> i64 {
                    self as i64
                }

                pub fn value(self) -> \(valueType) {
                    match self {
            \(valueArms.indented.indented.indented)
                    }
                }
            }

            impl crate::valdi_rust::ValdiRustPromiseValue for \(definition.rustName) {
                fn resolve_with(resolver: &crate::valdi_rust::ValdiRustPromiseResolver, value: Self) {
                    (resolver.resolve_long)(resolver.context, value.into_ffi());
                }
            }

            impl crate::valdi_rust::ValdiRustObservableValue for \(definition.rustName) {
                type Observer = crate::valdi_rust::ValdiRustLongObservableObserver;
            }

            impl crate::valdi_rust::ValdiRustObserver<\(definition.rustName)> for crate::valdi_rust::ValdiRustLongObservableObserver {
                fn next(&self, value: \(definition.rustName)) {
                    crate::valdi_rust::ValdiRustLongObservableObserver::next(self, value.into_ffi());
                }

                fn release(&self) {
                    crate::valdi_rust::ValdiRustLongObservableObserver::release(self);
                }
            }

        """
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

        if isGeneratedRustEnumType(type) {
            return FFIType(name: "int64_t")
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
        if isGeneratedRustEnumType(type) {
            return FFIParameter(type: ffiType(for: type), prelude: "", expression: "static_cast<int64_t>(\(name))")
        }

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
        case .enum(_) where isGeneratedRustEnumType(returnType):
            return "return static_cast<\(cppTypeName)>(\(callExpression));\n"
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
        if isGeneratedRustEnumType(type) {
            return ("", "static_cast<\(cppTypeName)>(\(name))")
        }

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
        case .enum(_) where isGeneratedRustEnumType(returnType):
            return "return static_cast<int64_t>(\(callExpression));\n"
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

        if isGeneratedRustEnumType(valueType) {
            return "resolver.resolveLong = valdiRustPromiseResolveEnum<\(valueCppTypeName)>;"
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
        if isGeneratedRustEnumType(type) {
            return ("", "\(name).into_ffi()")
        }

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
        case .enum(_) where isGeneratedRustEnumType(returnType):
            return "\(rustUserTypeName(for: returnType))::from_ffi(\(resultName))"
        case .string:
            if returnType.isOptional {
                return "\(rustUserTypeName(for: returnType))::from_handle(\(resultName))"
            }
            return "\(resultName).into_string()"
        case .bytes:
            if returnType.isOptional {
                return "\(rustUserTypeName(for: returnType))::from_handle(\(resultName))"
            }
            return "\(resultName).into_vec()"
        case .double, .bool, .long:
            if returnType.isOptional {
                return "\(rustUserTypeName(for: returnType))::from_handle(\(resultName))"
            }
            return resultName
        default:
            if isLocalRustWrapperType(returnType) {
                return "\(rustUserTypeName(for: returnType))::from_owned_handle(\(resultName))"
            }
            return "\(rustUserTypeName(for: returnType))::from_handle(\(resultName))"
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

    private func moduleScopedABIType(_ type: String) -> String {
        return type.replacingOccurrences(of: "valdi_rust::", with: "crate::valdi_rust::")
    }

    private func rustTempIdentifier(base: String, suffix: String) -> String {
        return base.replacingOccurrences(of: "r#", with: "") + suffix
    }

    private func rustModelCallArgument(type: ValdiModelPropertyType,
                                       name: String) -> (prelude: String, expression: String) {
        if isGeneratedRustEnumType(type) {
            return ("", "\(name).into_ffi()")
        }

        switch type.unwrappingOptional {
        case .string:
            if type.isOptional {
                return ("", "\(name).as_handle()")
            }
            let viewName = rustTempIdentifier(base: name, suffix: "_view")
            return ("let \(viewName) = crate::valdi_rust::ValdiRustStringView::from_str(&\(name));", viewName)
        case .bytes:
            if type.isOptional {
                return ("", "\(name).as_handle()")
            }
            let viewName = rustTempIdentifier(base: name, suffix: "_view")
            return ("let \(viewName) = crate::valdi_rust::ValdiRustBytesView::from_slice(&\(name));", viewName)
        case .double, .bool, .long:
            if type.isOptional {
                return ("", "\(name).as_handle()")
            }
            return ("", name)
        case .promise:
            return ("", "\(name).as_handle().expect(\"Only incoming Valdi promise handles can be stored in generated Rust model helpers\")")
        default:
            return ("", "\(name).as_handle()")
        }
    }

    private func rustModelReturnExpression(type: ValdiModelPropertyType,
                                           resultName: String) -> String {
        switch type.unwrappingOptional {
        case .enum(_) where isGeneratedRustEnumType(type):
            return "\(rustUserTypeName(for: type))::from_ffi(\(resultName))"
        case .string:
            if type.isOptional {
                return "\(rustUserTypeName(for: type))::from_handle(\(resultName))"
            }
            return "\(resultName).into_string()"
        case .bytes:
            if type.isOptional {
                return "\(rustUserTypeName(for: type))::from_handle(\(resultName))"
            }
            return "\(resultName).into_vec()"
        case .double, .bool, .long:
            if type.isOptional {
                return "\(rustUserTypeName(for: type))::from_handle(\(resultName))"
            }
            return resultName
        case .promise:
            return "\(rustUserTypeName(for: type))::from_handle(\(resultName))"
        default:
            if isLocalRustWrapperType(type) {
                return "\(rustUserTypeName(for: type))::from_owned_handle(\(resultName))"
            }
            return "\(rustUserTypeName(for: type))::from_handle(\(resultName))"
        }
    }

    private func rustModelConstructorSymbolName(definition: RustModelDefinition) -> String {
        return rustSymbolName(methodName: "\(definition.rustName)_new")
    }

    private func rustModelGetterSymbolName(definition: RustModelDefinition, property: ValdiModelProperty) -> String {
        return rustSymbolName(methodName: "\(definition.rustName)_get_\(property.name)")
    }

    private func rustModelSetterSymbolName(definition: RustModelDefinition, property: ValdiModelProperty) -> String {
        return rustSymbolName(methodName: "\(definition.rustName)_set_\(property.name)")
    }

    private func rustModelHelperRustSource(definition: RustModelDefinition) throws -> String {
        let constructorSymbol = rustModelConstructorSymbolName(definition: definition)
        let constructorParameterNames = definition.model.properties.map { rustIdentifier($0.name) }
        let constructorParameterDeclarations = zip(definition.model.properties, constructorParameterNames).map {
            "\($0.1): \(rustUserTypeName(for: $0.0.type))"
        }.joined(separator: ", ")
        let constructorExternParameters = zip(definition.model.properties, constructorParameterNames).map {
            "\($0.1): \(moduleScopedABIType(rustFFIParameterType(for: $0.0.type)))"
        }.joined(separator: ", ")
        let constructorArguments = zip(definition.model.properties, constructorParameterNames).map {
            rustModelCallArgument(type: $0.0.type, name: $0.1)
        }
        let constructorPrelude = constructorArguments.map { $0.prelude }
            .filter { !$0.isEmpty }
            .map { "        \($0)" }
            .joined(separator: "\n")
        let constructorCallArguments = constructorArguments.map { $0.expression }.joined(separator: ", ")

        var externDeclarations = "fn \(constructorSymbol)(\(constructorExternParameters)) -> crate::valdi_rust::ValdiRustHandle;\n"
        var methods = """
                pub fn new(\(constructorParameterDeclarations)) -> Self {
            \(constructorPrelude)
                    let handle = unsafe { \(constructorSymbol)(\(constructorCallArguments)) };
                    Self::from_owned_handle(handle)
                }

            """

        for property in definition.model.properties {
            let getterSymbol = rustModelGetterSymbolName(definition: definition, property: property)
            let setterSymbol = rustModelSetterSymbolName(definition: definition, property: property)
            let getterReturnType = rustFFIReturnType(for: property.type) ?? "crate::valdi_rust::ValdiRustHandle"
            let setterParameterType = moduleScopedABIType(rustFFIParameterType(for: property.type))
            let rustPropertyName = rustIdentifier(property.name)
            let getterName = "get_\(rustPropertyName)"
            let setterName = "set_\(rustPropertyName)"
            let setterArgument = rustModelCallArgument(type: property.type, name: "value")
            let setterPrelude = setterArgument.prelude.isEmpty ? "" : "        \(setterArgument.prelude)\n"
            let rustReturnType = rustUserTypeName(for: property.type)

            externDeclarations += "fn \(getterSymbol)(value: crate::valdi_rust::ValdiRustHandle) -> \(moduleScopedABIType(getterReturnType));\n"
            externDeclarations += "fn \(setterSymbol)(value: crate::valdi_rust::ValdiRustHandle, field: \(setterParameterType));\n"

            methods += """
                pub fn \(getterName)(&self) -> \(rustReturnType) {
                    let result = unsafe { \(getterSymbol)(self.as_handle()) };
                    \(rustModelReturnExpression(type: property.type, resultName: "result"))
                }

                pub fn \(setterName)(&mut self, value: \(rustReturnType)) {
            \(setterPrelude)        unsafe { \(setterSymbol)(self.as_handle(), \(setterArgument.expression)); }
                }

            """
        }

        return """

            unsafe extern "C" {
            \(externDeclarations.indented)
            }

            impl \(definition.rustName) {
            \(methods.indented)
            }

        """
    }

    private func rustModelHelperThunkSource(definition: RustModelDefinition,
                                            typeGenerator: CppCodeGenerator) throws -> String {
        let namesByProperty = resolveModelPropertyNames(model: definition.model)
        let constructorSymbol = rustModelConstructorSymbolName(definition: definition)
        let constructorParameters = try definition.model.properties.map { property -> (ValdiModelProperty, CppModelPropertyName, String) in
            guard let propertyName = namesByProperty[property.name] else {
                throw CompilerError("Could not resolve generated C++ model property name for \(definition.model.tsType).\(property.name)")
            }

            let propertyTypeName = try typeGenerator.getTypeParser(type: property.type,
                                                                    namePaths: [definition.model.tsType, property.name],
                                                                    nameAllocator: PropertyNameAllocator.forCpp().scoped()).typeNameResolver.resolve(cppType.declaration.namespace)
            return (property, propertyName, propertyTypeName)
        }
        let constructorParameterDeclarations = constructorParameters.map {
            "\(ffiType(for: $0.0.type).name) \($0.1.constructorParameterName)"
        }.joined(separator: ", ")
        let constructorArguments = constructorParameters.map {
            callbackCallArgumentExpression(type: $0.0.type,
                                           cppTypeName: $0.2,
                                           name: $0.1.constructorParameterName)
        }
        let constructorPrelude = constructorArguments.map { $0.prelude }
            .filter { !$0.isEmpty }
            .map { $0.indented }
            .joined(separator: "\n")
        let constructorCallArguments = constructorArguments.map { $0.expression }.joined(separator: ", ")

        var source = """

            extern "C" ValdiRustHandle \(constructorSymbol)(\(constructorParameterDeclarations)) {
            \(constructorPrelude)
                return valdiRustBridgeRetainBoxedValue<\(definition.cppTypeName)>(\(definition.cppTypeName)(\(constructorCallArguments)));
            }

        """

        for (property, propertyName, propertyTypeName) in constructorParameters {
            let getterSymbol = rustModelGetterSymbolName(definition: definition, property: property)
            let setterSymbol = rustModelSetterSymbolName(definition: definition, property: property)
            let getterReturnType = ffiReturnType(for: property.type).name
            let setterParameterType = ffiType(for: property.type).name
            let setterArgument = callbackCallArgumentExpression(type: property.type,
                                                                cppTypeName: propertyTypeName,
                                                                name: "field")
            let setterPrelude = setterArgument.prelude.isEmpty ? "" : "\(setterArgument.prelude.indented)\n"

            source += """
                extern "C" \(getterReturnType) \(getterSymbol)(ValdiRustHandle value) {
                    auto *modelBox = Valdi::unsafeBridgeUnretained<ValdiRustBridgeBox<\(definition.cppTypeName)>>(value.ptr);
                    \(callbackReturnStatement(returnType: property.type,
                                               cppTypeName: propertyTypeName,
                                               callExpression: "modelBox->value.\(propertyName.getterName)()").indented)
                }

                extern "C" void \(setterSymbol)(ValdiRustHandle value, \(setterParameterType) field) {
                    auto *modelBox = Valdi::unsafeBridgeUnretained<ValdiRustBridgeBox<\(definition.cppTypeName)>>(value.ptr);
            \(setterPrelude)        modelBox->value.\(propertyName.setterName)(\(setterArgument.expression));
                }

            """
        }

        return source
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
            [[maybe_unused]] static void valdiRustPromiseResolveEnum(void *context, int64_t value) {
                valdiRustPromiseSetValue<T>(context, static_cast<T>(value));
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

            template<typename OnEventFn, typename T>
            [[maybe_unused]] static void valdiRustEnumObservableNext(void *context, int64_t value) {
                auto *box = Valdi::unsafeBridgeUnretained<ValdiRustBridgeBox<OnEventFn>>(context);
                box->value(::snap::valdi_modules::bridge_observables::BridgeObserverEvent::NEXT,
                           std::nullopt,
                           std::optional<T>(static_cast<T>(value)),
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

            template<typename Observable, typename Value, typename SubscribeRust>
            [[maybe_unused]] static Observable valdiRustMakeEnumBridgeObservable(SubscribeRust subscribeRust) {
                using OnEventFn = typename Observable::SubscribeOnEventFn;
                return valdiRustMakeBridgeObservable<Observable, ValdiRustLongObservableObserver>(subscribeRust, valdiRustEnumObservableNext<OnEventFn, Value>);
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
        var modelHelperThunks = ""
        var rustEnumHelpers = ""
        var rustModelHelpers = ""
        var rustHandleTypeDefinitions = Set<RustHandleTypeDefinition>()
        var rustGeneratedModelDefinitions = [String: RustModelDefinition]()
        var rustGeneratedEnumDefinitions = [String: RustEnumDefinition]()

        for property in exportedModule.model.properties {
            switch property.type {
            case .function(let parameters, let returnType, _, _, _):
                parameters.forEach { collectRustHandleTypeDefinitions(type: $0.type, definitions: &rustHandleTypeDefinitions) }
                collectRustHandleTypeDefinitions(type: returnType, definitions: &rustHandleTypeDefinitions)
                parameters.forEach { collectRustGeneratedModelDefinitions(type: $0.type, definitions: &rustGeneratedModelDefinitions) }
                collectRustGeneratedModelDefinitions(type: returnType, definitions: &rustGeneratedModelDefinitions)
                parameters.forEach { collectRustGeneratedEnumDefinitions(type: $0.type, definitions: &rustGeneratedEnumDefinitions) }
                collectRustGeneratedEnumDefinitions(type: returnType, definitions: &rustGeneratedEnumDefinitions)

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
                    if bridgeObservableReturn == .handle || bridgeObservableReturn == .enumValue {
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
                collectRustGeneratedModelDefinitions(type: property.type, definitions: &rustGeneratedModelDefinitions)
                collectRustGeneratedEnumDefinitions(type: property.type, definitions: &rustGeneratedEnumDefinitions)

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

        for definition in rustGeneratedModelDefinitions.values.sorted(by: { $0.rustName < $1.rustName }) {
            modelHelperThunks += try rustModelHelperThunkSource(definition: definition, typeGenerator: typeGenerator)
            rustModelHelpers += try rustModelHelperRustSource(definition: definition)
        }

        for definition in rustGeneratedEnumDefinitions.values.sorted(by: { $0.rustName < $1.rustName }) {
            rustEnumHelpers += rustEnumHelperRustSource(definition: definition)
        }

        generator.body.appendBody("""
            \(modelHelperThunks)

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
                                                           enumHelpers: rustEnumHelpers,
                                                           callbackWrappers: rustCallbackWrappers,
                                                           modelHelpers: rustModelHelpers,
                                                           functions: rustAdapterFunctions).utf8Data()),
                         groupingIdentifier: "\(bundleInfo.name).rust_bridge.rs",
                         groupingPriority: 0)
        ]
    }

    private func rustAdapterSource(typeDefinitions: String, enumHelpers: String, callbackWrappers: String, modelHelpers: String, functions: String) -> String {
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
            \(enumHelpers)
            \(callbackWrappers)
            \(modelHelpers)

                include!(env!("VALDI_RUST_USER_ROOT_PATH"));
            }
            \(functions)
            """.indented
    }
}
