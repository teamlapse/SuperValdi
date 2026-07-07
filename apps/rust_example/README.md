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

The displayed count is powered by a native `BridgeObservable<number>` returned
from Rust through the generated C++ adapter. `src/CounterStore.ts` converts that
native bridge observable to a Valdi RxJS `Observable`, and each tap calls Rust's
`increment()` implementation, which emits the next value to native subscribers.
The Rust side uses the generated `BehaviorSubject<Double>` helper, so the
example does not need to hand-write observer storage or unsubscribe
bookkeeping. It also includes string and bytes functions implemented as normal
Rust `String` and `Bytes` signatures, with ABI conversion generated around the
user code.

Build the iOS or Android app with:

```sh
bazel build //apps/rust_example:rust_example_app_ios
bazel build //apps/rust_example:rust_example_app_android
```
