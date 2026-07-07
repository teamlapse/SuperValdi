# Polyglot Modules

## Introduction

Polyglot modules are a special kind of Valdi modules that expose a TypeScript API backed by a foreign language implementation, like Objective-C, Swift, C++, Rust, Java or Kotlin. When a regular Valdi module depends on a polyglot module, it will also depend on its implementation in the separate language. Some example of usecases for polyglot modules include:

- Exposing a third party library written in another language like C into TypeScript.
- Providing a more optimized implementation of a function or module in another language.
- Implementing some functionality that requires access to dependencies that are not currently visible by TypeScript.

## Implementing a polyglot module

A polyglot module has two key components:

- It has some part of the TypeScript type definition implemented in a separate language.
- Its `valdi_module()` Bazel target in the `BUILD.bazel` depends on user defined Android, iOS or native dependencies.

A regular Valdi module can be turned into a polyglot module at any time. We will look at an example where we implement a `join(components: string[], delimiter: string)` function in Objective-C, Kotlin and then C++, that gets exposed to TypeScript. The rest of this document expects that we have a Valdi module where we can add files and modify its `BUILD.bazel` file.

### TypeScript definition

We start by declaring the TypeScript API of the module for which we want an implementation in a foreign language:

```ts
/* @ExportModule */

export const DEFAULT_DELIMITER: string;

export function join(components: string[], delimiter: string): string;
```

The `@ExportModule` annotation tells the Valdi compiler to generate Objective-C, Swift, and Kotlin bindings that will help with implementing the module. Note that the file needs to be a TypeScript definition (`.d.ts` file), at `my_module/src/Joiner.d.ts` for instance.

### Bazel file

The `valdi_module` Bazel rule that is used for defining Valdi modules exports three properties that can be used to help writing a polyglot modules:

- `android_deps`: Defines a list of `android_library` dependencies, implemented in a JVM language, that will be built and included on an Android build. This can be used to add a JVM based implementation when targeting Android.
- `ios_deps`: Defines a list of `apple_library` or `cc_library` dependencies, implemented in a native language like Objective-C, Swift, and will be built and included on an iOS build. This can be used to add a native implementation when targeting iOS.
- `native_deps`: Defines a list of `cc_library` dependencies, implemented in a native cross-platform language like C++, and will be built and included on all platforms like iOS, Android or Desktop. This can be used to add a native and cross-platform implementation.
- `rust_deps`: Defines a list of Rust-backed native dependencies, typically produced by `valdi_rust_native_module()`. These dependencies are appended to `native_deps` and use the generated C++ polyglot ABI as the stable registration and marshalling layer.
- `web_deps`: Defines JavaScript or TypeScript web-native module implementations.
- `web_wasm_deps`: Defines Rust/WebAssembly web-native module implementations, typically produced by `valdi_rust_web_deps()`. These dependencies are appended to `web_deps` and should expose a JavaScript loader next to its `.wasm` file.

In a typical implementation, either `native_deps` is used, or `android_deps` and `ios_deps` are used. If the implementation is native cross-platform, like when writing bindings for a native library like `zstd`, `native_deps` would be used. If the implementation is platform dependent, like when writing bindings for a Camera API, `android_deps` and `ios_deps` would be used.

Rust native implementations should use `rust_deps` when the Rust crate links into the native app, and `web_wasm_deps` when the Rust crate compiles to WebAssembly for the web package.

### iOS implementation

We start by adding a `objc_library` target in the `BUILD.bazel` file of the module, which will contain the iOS specific implementation of the polyglot module:

```python
objc_library(
    name = "my_module_ios_impl",
    srcs = glob([
        "ios/**/*.m",
    ]),
    hdrs = [],
    copts = ["-I."],
    deps = [
        # Depend on the generated Objective-C API of the module
        ":my_module_api_objc",
        # Depend on the Valdi runtime
        "//valdi",
    ],
)

```

The implementation target will need to be referenced as `ios_deps` on the `valdi_module` target of the `BUILD.bazel` representing the module:

```python
valdi_module(
    name = "my_module",
    ios_deps = [":my_module_ios_impl"],
    ios_module_name = "SCCMyModule",
    ios_output_target = "release",
    # Etc...
)
```

We then write an Objective-C implementation file:

```objc
#import "valdi_core/SCValdiModuleFactoryRegistry.h"
#import <SCCMyModuleTypes/SCCMyModuleTypes.h>
#import <Foundation/Foundation.h>

@interface SCCMyModuleJoinerModuleImpl: NSObject<SCCMyModuleJoinerModule>

@end

@implementation SCCMyModuleJoinerModuleImpl

- (NSString *)DEFAULT_DELIMITER
{
    return @" ";
}

- (void)setDEFAULT_DELIMITER:(NSString *)defaultDelimiter
{

}

- (NSString *)joinWithComponents:(NSArray<NSString *> *)components delimiter:(NSString *)delimiter
{
  return [components componentsJoinedByString:delimiter];
}

@end

@interface SCCMyModuleJoinerModuleFactoryImpl : SCCMyModuleJoinerModuleFactory

@end

@implementation SCCMyModuleJoinerModuleFactoryImpl

// Registers the module into the Valdi runtime
VALDI_REGISTER_MODULE()

- (id<SCCMyModuleJoinerModule>)onLoadModule
{
    // Return the module implementation. Will be called lazily when the module
    // is imported for the first time.
    return [SCCMyModuleJoinerModuleImpl new];
}

@end
```

That's it! When TypeScript imports the `.d.ts` file, on iOS the `onLoadModule` of `SCCMyModuleJoinerModuleFactoryImpl` will be called, which will return the module instance that backs the implementation of the module.

### Android implementation

We start by adding a `valdi_android_library` target in the `BUILD.bazel` file of the module, which will contain the Android specific implementation of the polyglot module:

```python
load("//bzl/valdi:valdi_android_library.bzl", "valdi_android_library")

valdi_android_library(
    name = "my_module_android_impl",
    srcs = glob([
        "android/**/*.kt",
    ]),
    deps = [
        # Depend on the generated Kotlin API of the module
        ":my_module_api_kt",
        # Depend on the Valdi runtime
        "//valdi:valdi_java",
    ],
)

```

The implementation target will need to be referenced as `android_deps` on the `valdi_module` target of the `BUILD.bazel` representing the module:

```python
valdi_module(
    name = "my_module",
    android_class_path = "com.snap.valdi.modules.my_module",
    android_deps = [":my_module_android_impl"],
    android_output_target = "release",
    # Etc...
)
```

We then write a Kotlin implementation file:

```kotlin
package com.snap.valdi.modules.my_module

import com.snapchat.client.valdi_core.ModuleFactory
import com.snap.valdi.modules.RegisterValdiModule
import com.snap.valdi.modules.my_module.MyJoinerModule
import com.snap.valdi.modules.my_module.MyJoinerModuleFactory
import com.snap.valdi.modules.hello_world.NativeModuleModule

// Registers the module into the Valdi runtime
@RegisterValdiModule
class MyJoinerModuleFactoryImpl: MyJoinerModuleFactory() {

    override fun onLoadModule(): NativeModuleModule {
          // Return the module implementation. Will be called lazily when the module
          // is imported for the first time. We use an anonymous class here, but a
          // proper subclass can also be used of course.
          return object: MyJoinerModule {
            override val DEFAULT_DELIMITER = " "
            override fun join(components: List<String>, delimiter: String) -> String {
              return components.joinToString(delimiter)
            }
        }
    }
}
```

That's it! When TypeScript imports the `.d.ts` file, on Android the `onLoadModule` of `MyJoinerModuleFactoryImpl` will be called, which will return the module instance that backs the implementation of the module. Please note that for the `@RegisterValdiModule` annotation to work, the Kotlin file needs to be compiled through a `valdi_android_library` rule and that rule processes the annotations at build time to add some required information at runtime that is used to register the module.

### C++ implementation

We start by adding a `cc_library` target in the `BUILD.bazel` file of the module, which will contain the native implementation of the polyglot module:

```python
cc_library(
    name = "my_module_native_impl",
    srcs = glob([
        "native/**/*.cpp",
    ]),
    # Required for automatic registration of the module into the Valdi runtime
    alwayslink = 1,
    deps = [
        # Depend on the generated C++ API of the module
        ":my_module_cpp",
    ],
)

```

The implementation target will need to be referenced as `native_deps` on the `valdi_module` target of the `BUILD.bazel` representing the module:

```python
valdi_module(
    name = "my_module",
    native_deps = [":my_module_native_impl"],
    # Etc...
)
```

We then write a C++ implementation file. The Valdi compiler generates the typed C++ module interface and factory base from the TypeScript declaration; the implementation only needs to subclass those generated types:

```c++
#include "valdi_modules/my_module/my_module.hpp"

using namespace Valdi;

namespace snap::valdi_modules::my_module {

class JoinerModuleFactoryImpl: public JoinerModuleFactory {
public:
    JoinerModuleFactoryImpl() = default;
    ~JoinerModuleFactoryImpl() override = default;

    Ref<JoinerModule> onLoadModule() final {
        class JoinerModuleImpl: public JoinerModule {
        public:
            JoinerModuleImpl() = default;
            ~JoinerModuleImpl() override = default;

            StringBox DEFAULT_DELIMITER() final {
                return StringBox::fromCString(" ");
            }

            StringBox join(const std::vector<StringBox> &components, const StringBox &delimiter) final {
                return StringBox::join(components, delimiter);
            }
        };

        return makeShared<JoinerModuleImpl>();
    }
};

// Register our module to the Valdi runtime
auto kRegisterModule = RegisterModuleFactory::registerTyped<JoinerModuleFactoryImpl>();

}
```

### Rust implementation

Rust native modules use the same TypeScript source of truth as Objective-C, Swift, Kotlin, Java, and C++ modules. The `@ExportModule` declaration drives Valdi's generated C++ API, and when `rust_deps` is present the compiler also emits a generated `{module}_rust_bridge` C++ adapter. The Rust crate implements the generated C ABI symbols; users do not write C++ adapter classes.

Start with a TypeScript declaration, for example in `my_module/src/Math.d.ts`:

```ts
/* @ExportModule */

export function add(lhs: number, rhs: number): number;
```

Add a Rust-backed native implementation target:

```python
load("//bzl/valdi:valdi_module.bzl", "valdi_module")
load("//bzl/valdi:valdi_rust_module.bzl", "valdi_rust_native_module")

valdi_rust_native_module(
    name = "my_module_rust_impl",
    module = ":my_module",
    rust_srcs = glob(["rust/**/*.rs"]),
    crate_root = "rust/lib.rs",
    crate_name = "my_module_rust",
)

valdi_module(
    name = "my_module",
    srcs = glob(["src/**/*.ts", "src/**/*.tsx"]),
    rust_deps = [":my_module_rust_impl"],
    # other module settings...
)
```

Because the declaration above is in `Math.d.ts`, the generated native module type is `MathModule`. For module `my_module`, the generated bridge calls this Rust symbol:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn valdi_rust_my_module_math_module_add(lhs: f64, rhs: f64) -> f64 {
    lhs + rhs
}
```

The generated bridge derives symbol names as:

```text
valdi_rust_<module_name>_<generated_module_type>_<method_name>
```

with each component converted to snake case.

Primitive values use Rust aliases (`number` -> `Double`, `boolean` -> `Bool`, `long` -> `Long`). Strings use Rust `String`, and bytes use the generated `Bytes` alias (`Vec<u8>`). The generated adapter converts those Rust-authored types to and from the concrete C ABI views and owned values. Generated models, proxies, promises, callbacks, and other converter-backed native values are passed as `ValdiRustHandle` values with retain/release callbacks so Rust can explicitly control lifetime when it stores a handle beyond the current call.

For handle-backed values, the generated Rust adapter emits typed opaque wrappers into the user module before including the Rust source. A TypeScript model like this:

```typescript
// @ExportModel
export interface CounterPayload {
  label: string;
  value: number;
}

// @ExportFunction
export function echoPayload(payload: CounterPayload): CounterPayload;
```

is authored in Rust using the generated `CounterPayload` type, not a raw ABI handle:

```rust
pub fn echo_payload(payload: CounterPayload) -> CounterPayload {
    payload
}
```

The wrapper preserves the same native value across the boundary and exposes `retain()`, unsafe `release()`, `as_handle()`, and `into_handle()` for lifetime-sensitive storage or pass-through code. Rich Rust property/method helpers for generated models are still a later parity step.

For example, TypeScript declarations using strings and bytes:

```typescript
// @ExportFunction
export function formatCount(prefix: string, value: number): string;

// @ExportFunction
export function labelBytes(label: string): Uint8Array;

// @ExportFunction
export function payloadSize(payload: Uint8Array): number;
```

are implemented with normal Rust types:

```rust
pub fn format_count(prefix: String, value: Double) -> String {
    format!("{}: {}", prefix, value as i64)
}

pub fn label_bytes(label: String) -> Bytes {
    label.into_bytes()
}

pub fn payload_size(payload: Bytes) -> Double {
    payload.len() as Double
}
```

### Rust WebAssembly implementation

For web, compile Rust to a `.wasm` artifact and provide a JavaScript loader under the module's web native surface. Package both files with `valdi_rust_web_deps()` and pass that target to `web_wasm_deps`:

```python
load("//bzl/valdi:valdi_rust_module.bzl", "valdi_rust_web_deps")

valdi_rust_web_deps(
    name = "my_module_rust_web",
    module_name = "my_module",
    js_loader = [":MathWasm.js"],
    wasm = [":math_wasm_bg.wasm"],
    dts = [":MathWasm.d.ts"],
)

valdi_module(
    name = "my_module",
    srcs = glob(["src/**/*.ts", "src/**/*.tsx"]),
    web_wasm_deps = [":my_module_rust_web"],
    # other module settings...
)
```

The web packager places the files under `native/my_module/web/`. The JavaScript loader should load the `.wasm` file using a relative path so it continues to work after Valdi collapses native web paths. The loader is registered the same way as other web polyglot modules, so imports from TypeScript continue to resolve through the generated web native-module shims.

### Observable interop

Valdi does not export RxJS `Observable<T>` directly as a native ABI type. Use the `bridge_observables` module:

- TypeScript APIs expose `Observable<T>` or `Subject<T>` through the `@NativeTypeConverter` definitions in `bridge_observables`.
- Native APIs receive generated `BridgeObservable<T>`, `BridgeObserver<T>`, or `BridgeSubject<T>` models.
- Rust implementations return a Rust observable source. The generated adapter subscribes that source to the generated native `BridgeObservable<T>` ABI.
- The Rust support module provides `Observable<T>` and `BehaviorSubject<T>` aliases for common copyable observable state. It also provides callback-observable aliases for direct observer-backed sources.

For example, this TypeScript declaration:

```typescript
import { BridgeObservable } from 'bridge_observables/src/types/BridgeObservable';

/** @ExportModule */
// @ExportFunction
export function count(): BridgeObservable<number>;

// @ExportFunction
export function increment(): void;
```

can be implemented in Rust without hand-writing an observer registry:

```rust
use std::sync::OnceLock;

static COUNT: OnceLock<BehaviorSubject<Double>> = OnceLock::new();

fn count_subject() -> Observable<Double> {
    COUNT.get_or_init(|| BehaviorSubject::new(0.0))
}

pub fn count() -> Observable<Double> {
    count_subject()
}

pub fn increment() {
    let count = count_subject();
    count.next(count.get() + 1.0);
}
```

`BridgeObservable<T>` is accepted only when `T` is one of the native boundary shapes Valdi already supports: primitives, strings, bytes, arrays/maps of supported types, generated native models, marshall-as-untyped native values, or values with an existing `@NativeTypeConverter`. Unsupported payload types fail during compilation.

This is the same interop path used by Objective-C, Swift, Kotlin, Java, and C++ implementations, and it keeps unsubscribe, error, and completion semantics consistent with RxJS.

That's it! When TypeScript imports the `.d.ts` file, the generated module factory will load the Rust-backed implementation registered through the generated bridge.
