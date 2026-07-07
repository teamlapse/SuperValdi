# Rust Native Module Example

This example shows a Rust-backed Valdi native module whose bridge is generated
from the TypeScript `@ExportModule` declaration in `src/RustCounter.d.ts`.

The app code imports ordinary TypeScript functions:

```ts
import { count, increment } from './RustCounter';
```

The Valdi compiler emits the C++ adapter and Rust crate adapter for that
TypeScript surface when `valdi_module(rust_deps = [...])` is set. The Rust
implementation exports ordinary Rust functions; the generated adapter owns the
C ABI symbols.

The displayed count is exposed to TypeScript as a Valdi RxJS
`Observable<number>`, converted through the existing `bridge_observables`
native type converter, and powered by a native observable source returned from
Rust. Each tap calls Rust's `describeAfterIncrement()` implementation, which
emits the next value to native subscribers and calls a TypeScript formatter
through a generated Rust callback wrapper. The Rust side uses the generated
`BehaviorSubject<Double>` helper, so the example does not need to hand-write
observer storage or unsubscribe bookkeeping. The app also renders a
`Promise<string>` returned from Rust via `Promise::resolved(...)`. It includes
string and bytes functions implemented as normal Rust `String` and `Bytes`
signatures, with ABI conversion generated around the user code. The
`CounterPayload` model is also generated as a Rust wrapper with `new()`,
`get_label()`, and `get_value()` helpers; the app sends a TypeScript object into
Rust and displays the model Rust returns. `CounterMode` demonstrates generated
Rust enum helpers: Rust receives a normal enum value and can read the original
TypeScript enum value through `value()`.

The example also includes a web-native Rust/WASM implementation. The
`rust_counter_web_wasm` target produces the `.wasm` artifact, and
`valdi_rust_web_deps()` packages it next to the CommonJS loader under
`native/rust_example/web/`. The generated web package registers that loader as
`rust_example/src/RustCounter`, so the same TypeScript import works on web.

Build the iOS, Android, desktop, or web package with:

```sh
bazel build //apps/rust_example:rust_example_app_ios
bazel build //apps/rust_example:rust_example_native_android
bazel build //apps/rust_example:rust_example_native_desktop
bazel build //apps/rust_example:rust_example_npm
```
