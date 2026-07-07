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
signatures, with ABI conversion generated around the user code.

Build the iOS or Android app with:

```sh
bazel build //apps/rust_example:rust_example_app_ios
bazel build //apps/rust_example:rust_example_native_android
```
