# Rust-First SuperValdi Production Migration

## Intent

SuperValdi becomes a production Rust-first cross-platform app framework for iOS, Android, and web. Rust owns app UI, view models, state, actions, business logic, UI IR, runtime diffing, hot reload, dynamic UI ingestion, backend operation modeling, and the long-term framework core.

Existing Valdi TypeScript, JS runtime, TSN, and C++ rendering systems remain as compatibility infrastructure for existing TypeScript Valdi apps. Rust-authored apps do not depend on TypeScript, TSX, JS rendering, `RenderRequest`, `ViewNodeRenderer`, `ViewNodeTree`, C++ renderer adapter targets, or transition adapter targets in the final production build graph.

## Production Definition

- Rust developers write app UI, state, view models, actions, dynamic UI producers, and module/view contracts in Rust.
- Rust-authored iOS apps render with the Rust UIKit backend by default.
- Rust-authored Android apps render with the Rust Android View backend by default.
- Rust-authored web apps render with the Rust DOM backend by default.
- Rust-authored app build graphs contain no TypeScript compiler/runtime dependency and no JS direct renderer, TSN C emitter/runtime, `RenderRequest`, `ViewNodeRenderer`, `ViewNodeTree`, C++ renderer adapter target, or transition adapter target dependency.
- UI IR represents every production surface listed in the Replacement Coverage Summary before any platform backend is allowed to claim production readiness.
- UI IR hot reload updates UI structure, layout, styling, text, assets, bindings, event wiring, module references, and native view references without app reinstall or relaunch.
- Rust logic hot patching covers the declared action-body edit class; every unsupported Rust edit returns an exact rebuild-required reason from the dev server.
- Rust-authored polyglot module contracts generate Swift, Kotlin, JS/DOM, Rust-host, C++ transition-compatible, and TS compatibility factories. The generator implementation language follows existing repo infrastructure; the app-facing contract is Rust.
- Rust-authored native view contracts generate Swift/UIKit, Kotlin/View, JS/DOM, Rust metadata, PNG fallback, lifecycle, measurement, reuse, event, accessibility, and platform-extension stubs.
- PNG rendering consumes the same validated UI IR and produces deterministic golden outputs for every static-render fixture.
- Existing TypeScript Valdi apps continue to build and render through the retained compatibility path.

## No-Gaps Rules

- Required IR surfaces have concrete schema, fixture, validator, backend-op, owner PR, and proof PR before production default.
- Required surfaces are never represented only by an opaque platform bag. Platform extensions are typed, named, versioned, and assigned to an owner PR.
- Backend production support requires passing the shared conformance fixture for that surface.
- Production default requires build graph checks proving Rust-authored apps are off the TypeScript and C++ renderer paths.
- App developers write no C or C++ bridge code. Internal generated C ABI shims are allowed only at Rust-to-platform FFI boundaries and are not app APIs.
- Release-blocking CI gates and performance budgets land before the production default switch.

## Current Repo Facts Driving The Plan

- `compiler/companion/src/JSXProcessor.ts` transforms TSX into imperative renderer calls and owns TSX concerns such as keys, slots, refs, context, lazy nodes, spread attrs, static attrs, dynamic attrs, and component injection.
- `src/valdi_modules/src/valdi/valdi_core/src/JSXRendererDelegate.ts` batches create, destroy, root, move, attribute, animation, layout, draw, visibility, and frame observer entries into a binary buffer.
- `valdi/src/valdi/runtime/Rendering/RenderRequestEntries.hpp` defines the current native operation family: create, destroy, move, root, set attribute, start/end/cancel animations, layout callback, and next draw callback.
- `valdi/src/valdi/runtime/Rendering/RenderRequest.hpp` stores render entries plus visibility and frame observer callbacks.
- `valdi/src/valdi/runtime/Rendering/ViewNodeRenderer.cpp` applies render entries to `ViewNodeTree`, `ViewNode`, attributes, CSS updates, layout callbacks, draw callbacks, and native animator attachment.
- `valdi/src/valdi/runtime/Interfaces/IViewManager.hpp` owns platform identity, view factory creation, action calls, native animator creation, class hierarchy, attribute binding, point scale, native wrappers, and transaction creation.
- `valdi/src/valdi/runtime/Interfaces/IViewTransaction.hpp` owns platform mutations: root updates, moving views, child insertion/removal, layout invalidation, frames, scroll specs, loaded assets, view layout, animation flush/cancel, pooling, snapshots, next draw, and transaction-thread execution.
- `valdi/src/valdi/runtime/Context/ViewNodeTree.hpp` owns root view node state, layout specs, measuring, viewport, updates, animation tokens, CSS updates, view factories, visibility/frame observers, asset tracking, next layout, and next draw.
- `valdi/src/valdi/runtime/Attributes/AttributesManager.hpp` binds class attributes through preprocessors, postprocessors, Yoga config, color palette, default handlers, scroll attributes, and measure delegates.
- `valdi/src/valdi/runtime/Attributes`, `CSS`, `Resources`, `Text`, and `Views` contain production surfaces the Rust path covers: accessibility, animation, asset loading, border/radius, composite attrs, Yoga, CSS, loaded assets, text parsing, measure delegates, view factories, preloading, and transactions.
- `compiler/compiler/Compiler/Sources/Processors/HotReloadingProcessor.swift` hot reloads debug compiled sources and asset packages; it ignores native sources.
- `valdi/src/valdi/runtime/JavaScript/JavaScriptRuntime.cpp` records native module hot reload as missing in the TSN path.
- `compiler/companion/src/native/emitter/CompilerNativeCEmitter.ts` emits TSN C code and includes `tsn/tsn.h`; Rust-authored production apps do not depend on this path.
- `compiler/compiler/Compiler/Sources/Generation/Cpp/CppModuleGenerator.swift`, `ObjCModuleGenerator.swift`, and `KotlinModuleGenerator.swift` generate module factories for C++, Objective-C/Swift-facing iOS, and Kotlin/Android.
- `compiler/compiler/Compiler/Sources/Pipeline/CompilationItem.swift` already models `ios`, `android`, `web`, and `cpp` platforms.
- `compiler/compiler/Compiler/Sources/Processors/PrependWebJsProcessor.swift` rewrites web JS requires and module paths for the current web runtime.
- `compiler/compiler/Compiler/Sources/Images/ImageVariantResolver.swift` already resolves iOS, Android, web, and SVG image variants.
- `MODULE.bazel` already configures `rules_rust`, Swift, Android, Kotlin, JS/TS, Apple, C++, and existing crate-universe use.

## Replacement Coverage Summary

PR01 creates the authoritative `docs/rust_migration/replacement_contract.yaml` and generated `docs/rust_migration/replacement_contract.md`. This table is the PR00 planning seed for those contract rows; PR01 turns each row into machine-readable owner and proof data consumed by PR03, PR04, PR05, PR06, PR36, PR37, and PR38.

| Surface | Contract Row Coverage | Implemented By | Proved By |
| --- | --- | --- | --- |
| Schema and versioning | schema.version, feature flags, capability IDs, binary/JSON versions | PR01,03,05 | PR05,36 |
| Component identity | component IDs, node IDs, keys, state identity, source spans, hot reload identity, dynamic identity | PR01,03,07,11,13 | PR07,11,36 |
| Tree structure | root, parent order, keyed movement, fragments, slots, portals, contexts, destruction, pooling | PR01,03,06,26 | PR26,36 |
| Element taxonomy | view, layout, scroll, image, text, rich text, text input, controls, list, webview, native view, drawing host | PR01,03,04,16,26 | PR04,16,36 |
| Layout | Yoga/flex, absolute, min/max, measure, safe area, scroll sizing, z order, clipping, transform, RTL | PR01,03,06,27 | PR27,36 |
| Styling | backgrounds, colors, opacity, border, radius, shadow, overflow, visibility, display, transform, class metadata, platform extensions | PR01,03,06,27 | PR27,36 |
| Text | plain text, rich spans, fonts, wrapping, truncation, links, measure hooks, input value, selection, composition | PR01,03,04,30 | PR30,36 |
| Assets | local assets, variants, remote refs, data refs, resize modes, loading state, errors, animated metadata, cache identity | PR01,03,04,29 | PR29,36 |
| Events and gestures | tap, press, long press, pan, scroll, focus, blur, input, keyboard submit, layout, draw, visibility, frame observer, custom native events | PR01,03,08,32 | PR32,36 |
| Actions and state | sync actions, async actions, results, typed errors, cancellation, invalidation, scheduling, coalescing | PR01,03,08,11,12 | PR08,12,36 |
| Bindings and expressions | field paths, literals, nullable values, logic, comparisons, lists, computed projections, platform constants, source spans | PR01,03,08,09,11 | PR08,09,11,36 |
| Animations | start, end, cancel, timing, easing, delay, repeat, fill mode, property animation, layout animation, transaction grouping | PR01,03,06,33 | PR33,36 |
| Native modules | Rust contracts, complete type matrix, dispatch targets, Swift/Kotlin/JS/Rust/C++ transition/TS factories | PR01,18,19,20,21 | PR21,36 |
| Native views | Rust contracts, typed attrs, typed events, lifecycle, measurement, reuse, platform extensions, accessibility, fallback | PR01,22,23,34 | PR23,34,36 |
| Accessibility | role, label, hint, value, state, action, focus order, grouping, hidden state, platform overrides | PR01,03,31 | PR31,36 |
| Hot reload | patch identity, source spans, compatibility, assets, bindings, actions, module refs, native view refs | PR01,11,12 | PR11,12,36 |
| Diagnostics | schema paths, source spans, backend paths, capability errors, unsupported attrs/events, fixture IDs, action IDs, module IDs | PR01,05,06,27 | PR05,27,36 |
| Web DOM | DOM mapping, CSS/style mapping, events, text measurement, class emission, browser snapshots | PR01,16,28 | PR16,36 |
| PNG backend | layout snapshot, display list, text, image, native/webview fallback, accessibility debug metadata | PR01,17,25 | PR17,25,36 |
| Dynamic UI | validated IR from Rust DSL, generated fixtures, JSON debug, binary bytes, in-memory producers | PR01,05,13 | PR13,36 |
| TS compatibility | TSX coverage map, direct renderer compatibility, TSX-to-IR equivalence, no Rust-path TS dependency | PR01,10,36,38 | PR10,36,38 |
| Build graph | Bazel targets, crate graph, generated glue, platform app targets, compatibility labels, forbidden dependency checks | PR01,02,14,15,16,18,19,20,22,23,37,38 | PR02,37,38 |

## Stack Ordering Rationale

The stack does not implement IR first. It first creates the production replacement contract, then adds Rust/Bazel scaffolding, then implements the full IR schema against that contract. This prevents a partial IR from becoming the architecture.

1. Contract and build foundation before IR.
2. IR schema and fixtures before serializer, validator, and backend ops.
3. TSX-to-IR compatibility before hot reload and backend replacement, so schema gaps surface early.
4. Runtime, state, DSL, hot reload, Rust hot patch, and dynamic UI before app samples.
5. Transition adapters before Rust backend replacement to prove behavior against existing iOS and Android surfaces quickly.
6. Web DOM and PNG are first-class backends before product samples.
7. Polyglot modules and native views land as contract, factory, and conformance layers before samples.
8. Rust platform backends replace the native C++ renderer path by surface family: tree, attrs/layout, transactions, assets, text/input, accessibility, events, animations, then native view handoff.
9. Release gates and performance budgets land before the production default switch.
10. The production default switch lands only after full parity and build graph exclusion checks pass.

## Stack Map

| Order | Branch | PR Title | Step Detail | Purpose | Depends On | Ownership | Proof Point | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 00 | `codex/rust-supervaldi-00-plan` | plan: Rust-first production migration | [00-plan.md](2026-07-07-rust-first-supervaldi/steps/00-plan.md) | Add the production migration plan and separate step detail files only. | `main` | Docs | Plan PR contains docs only. | In progress |
| 01 | `codex/rust-supervaldi-01-contract` | feat: add production replacement contract | [01-contract.md](2026-07-07-rust-first-supervaldi/steps/01-contract.md) | Create the authoritative contract consumed by every implementation PR. | PR00 | Docs + test metadata | Contract check passes. | Complete |
| 02 | `codex/rust-supervaldi-02-bazel-rust-foundation` | build: add Rust framework Bazel foundation | [02-bazel-rust-foundation.md](2026-07-07-rust-first-supervaldi/steps/02-bazel-rust-foundation.md) | Add the Rust workspace and Bazel foundation used by all Rust framework work. | PR01 | Bazel + Rust skeleton | `bazelisk test` passes for the Rust foundation labels. | Complete |
| 03 | `codex/rust-supervaldi-03-ir-schema` | feat: add full Rust UI IR schema | [03-ir-schema.md](2026-07-07-rust-first-supervaldi/steps/03-ir-schema.md) | Implement Rust UI IR types for every production surface in the contract. | PR02 | Rust | Schema coverage and public examples tests prove no contract row lacks a Rust type. | Complete |
| 04 | `codex/rust-supervaldi-04-fixture-corpus` | test: add complete IR fixture corpus | [04-fixture-corpus.md](2026-07-07-rust-first-supervaldi/steps/04-fixture-corpus.md) | Add fixture coverage for every contract row and parity target. | PR03 | Rust fixtures + test metadata | Fixture coverage test proves every contract row and fixture tag pair has coverage. | Complete |
| 05 | `codex/rust-supervaldi-05-codec-validator` | feat: add IR codec validator inspector | [05-codec-validator.md](2026-07-07-rust-first-supervaldi/steps/05-codec-validator.md) | Add production and debug encoding plus validation tooling. | PR04 | Rust | Every fixture round-trips through JSON and binary codecs. | Planned |
| 06 | `codex/rust-supervaldi-06-backend-ops` | feat: add full backend operation model | [06-backend-ops.md](2026-07-07-rust-first-supervaldi/steps/06-backend-ops.md) | Create the backend operation contract shared by native, web, PNG, and transition renderers. | PR05 | Rust | Mock backend snapshots cover every operation family. | Planned |
| 07 | `codex/rust-supervaldi-07-runtime-tree-diff` | feat: add Rust runtime tree diff | [07-runtime-tree-diff.md](2026-07-07-rust-first-supervaldi/steps/07-runtime-tree-diff.md) | Add Rust document loading, identity preservation, and render-op diffing. | PR06 | Rust | Static fixture declarations emit expected ordered backend ops. | Planned |
| 08 | `codex/rust-supervaldi-08-state-bindings-actions` | feat: add state bindings and action scheduler | [08-state-bindings-actions.md](2026-07-07-rust-first-supervaldi/steps/08-state-bindings-actions.md) | Add Rust state, binding evaluation, and action dispatch. | PR07 | Rust | State fixtures update predictably. | Planned |
| 09 | `codex/rust-supervaldi-09-rust-ui-dsl` | feat: add complete Rust UI DSL | [09-rust-ui-dsl.md](2026-07-07-rust-first-supervaldi/steps/09-rust-ui-dsl.md) | Add the Rust authoring surface for declaring UI. | PR08 | Rust | DSL output equals canonical IR for every contract row. | Planned |
| 10 | `codex/rust-supervaldi-10-tsx-to-ir-compat` | feat: add TSX-to-IR compatibility frontend | [10-tsx-to-ir-compat.md](2026-07-07-rust-first-supervaldi/steps/10-tsx-to-ir-compat.md) | Preserve existing TypeScript apps and surface schema gaps before backend replacement begins. | PR09 | TS compatibility + Rust IR | TSX fixtures produce normalized IR equal to Rust DSL fixtures. | Planned |
| 11 | `codex/rust-supervaldi-11-ir-hot-reload` | feat: add IR hot reload pipeline | [11-ir-hot-reload.md](2026-07-07-rust-first-supervaldi/steps/11-ir-hot-reload.md) | Deliver fast UI hot reload through IR patches. | PR10 | Rust tooling + debug host hooks | UI tree, style, layout, text, asset, binding, event, accessibility, module ref, and native view ref edits patch live on the sample host without reinstall or relaunch. | Planned |
| 12 | `codex/rust-supervaldi-12-rust-hot-patch` | feat: add Rust logic hot patch support | [12-rust-hot-patch.md](2026-07-07-rust-first-supervaldi/steps/12-rust-hot-patch.md) | Add the declared Rust logic edit class for live development. | PR11 | Rust tooling | Supported action-body edit patches live. | Planned |
| 13 | `codex/rust-supervaldi-13-dynamic-ui` | feat: add dynamic IR ingestion API | [13-dynamic-ui.md](2026-07-07-rust-first-supervaldi/steps/13-dynamic-ui.md) | Allow validated UI IR to be produced outside the Rust DSL and consumed by the same runtime contract. | PR11 | Rust | Dynamic producer feeds validator, runtime, and mock backend using the same diagnostics as DSL-produced IR. | Planned |
| 14 | `codex/rust-supervaldi-14-ios-transition` | feat: render Rust IR through retained iOS Valdi backend | [14-ios-transition.md](2026-07-07-rust-first-supervaldi/steps/14-ios-transition.md) | Prove Rust IR can render through the retained iOS Valdi backend while the Rust UIKit backend is built. | PR13 | Rust + C++ transition + iOS host hook | iOS simulator renders contract fixtures through retained backend. | Planned |
| 15 | `codex/rust-supervaldi-15-android-transition` | feat: render Rust IR through retained Android Valdi backend | [15-android-transition.md](2026-07-07-rust-first-supervaldi/steps/15-android-transition.md) | Prove Rust IR can render through the retained Android Valdi backend while the Rust Android backend is built. | PR14 | Rust + Kotlin + C++ transition | Android emulator renders contract fixtures through retained backend. | Planned |
| 16 | `codex/rust-supervaldi-16-web-dom-backend` | feat: add Rust DOM backend | [16-web-dom-backend.md](2026-07-07-rust-first-supervaldi/steps/16-web-dom-backend.md) | Add the production web rendering path for Rust-authored apps. | PR13 | Rust/WASM + JS DOM host | Browser DOM snapshots pass for DOM-tagged fixtures. | Planned |
| 17 | `codex/rust-supervaldi-17-png-display-list` | feat: add deterministic PNG display-list backend | [17-png-display-list.md](2026-07-07-rust-first-supervaldi/steps/17-png-display-list.md) | Add deterministic raster output for the base static render surface. | PR13 | Rust | Golden PNGs pass for base static-render fixtures. | Planned |
| 18 | `codex/rust-supervaldi-18-polyglot-contract` | feat: add Rust-authored module contract IDL | [18-polyglot-contract.md](2026-07-07-rust-first-supervaldi/steps/18-polyglot-contract.md) | Add the app-facing Rust contract surface for native modules. | PR13 | Rust contract + generator metadata | Contract type matrix test passes. | Planned |
| 19 | `codex/rust-supervaldi-19-polyglot-ios-android` | feat: generate Swift and Kotlin module factories | [19-polyglot-ios-android.md](2026-07-07-rust-first-supervaldi/steps/19-polyglot-ios-android.md) | Generate native iOS and Android factories from Rust-authored module contracts. | PR18 | Rust contract + Swift/Kotlin generator output | Same module conformance passes on iOS and Android. | Planned |
| 20 | `codex/rust-supervaldi-20-polyglot-web-rust-transition` | feat: generate web Rust host and transition module factories | [20-polyglot-web-rust-transition.md](2026-07-07-rust-first-supervaldi/steps/20-polyglot-web-rust-transition.md) | Generate remaining module factories required by production and compatibility paths. | PR19 | Rust contract + JS/DOM + Rust host + C++ transition + TS compatibility | Same module conformance passes on web, Rust host, C++ transition, and TS compatibility targets. | Planned |
| 21 | `codex/rust-supervaldi-21-polyglot-conformance` | test: add complete module conformance suite | [21-polyglot-conformance.md](2026-07-07-rust-first-supervaldi/steps/21-polyglot-conformance.md) | Lock module behavior across every target before native view contracts land. | PR20 | Test infra across Rust, Swift, Kotlin, JS, C++ transition, and TS compatibility | Complete module conformance passes on iOS, Android, web, Rust host, C++ transition, and TS compatibility. | Planned |
| 22 | `codex/rust-supervaldi-22-native-view-contract` | feat: add Rust-authored native view contract IDL | [22-native-view-contract.md](2026-07-07-rust-first-supervaldi/steps/22-native-view-contract.md) | Add the app-facing Rust contract surface for native views. | PR21 | Rust contract + generator metadata | Native view contract matrix test passes. | Planned |
| 23 | `codex/rust-supervaldi-23-native-view-stubs` | feat: generate native view platform stubs | [23-native-view-stubs.md](2026-07-07-rust-first-supervaldi/steps/23-native-view-stubs.md) | Generate platform stubs and retained-path metadata from Rust-authored native view contracts. | PR22 | Rust contract + Swift + Kotlin + JS/DOM + PNG fallback metadata | Native view stub conformance passes for iOS, Android, web, PNG fallback metadata, and retained compatibility metadata. | Planned |
| 24 | `codex/rust-supervaldi-24-samples-transition` | feat: add Rust production sample suite on transition backends | [24-samples-transition.md](2026-07-07-rust-first-supervaldi/steps/24-samples-transition.md) | Add end-to-end samples before Rust platform backend replacement begins. | PR23 | Rust + Swift + Kotlin + JS + transition renderers | Samples build and launch on iOS, Android, and web. | Planned |
| 25 | `codex/rust-supervaldi-25-png-production-fixtures` | test: complete PNG production fixture coverage | [25-png-production-fixtures.md](2026-07-07-rust-first-supervaldi/steps/25-png-production-fixtures.md) | Complete PNG coverage for all static-render contract surfaces. | PR24 | Rust + deterministic assets | PNG golden suite passes on CI for every static-render fixture. | Planned |
| 26 | `codex/rust-supervaldi-26-rust-view-tree` | feat: add Rust native view tree | [26-rust-view-tree.md](2026-07-07-rust-first-supervaldi/steps/26-rust-view-tree.md) | Start replacing retained native rendering with Rust-owned tree management. | PR24 | Rust | Rust tree snapshots match retained backend snapshots for tree fixtures. | Planned |
| 27 | `codex/rust-supervaldi-27-rust-attrs-layout` | feat: port attrs layout and Yoga orchestration | [27-rust-attrs-layout.md](2026-07-07-rust-first-supervaldi/steps/27-rust-attrs-layout.md) | Port layout and attribute handling to the Rust backend. | PR26 | Rust + Yoga integration | Layout and attr metrics match retained backend for all layout fixtures. | Planned |
| 28 | `codex/rust-supervaldi-28-rust-platform-transactions` | feat: add Rust platform transactions | [28-rust-platform-transactions.md](2026-07-07-rust-first-supervaldi/steps/28-rust-platform-transactions.md) | Replace platform mutation submission for Rust backends. | PR27 | Rust + Swift/UIKit + Kotlin/View + JS/DOM | iOS, Android, and web mutate views from Rust backend transactions. | Planned |
| 29 | `codex/rust-supervaldi-29-rust-assets` | feat: port asset loading to Rust backend | [29-rust-assets.md](2026-07-07-rust-first-supervaldi/steps/29-rust-assets.md) | Port production asset resolution and loaded-asset propagation. | PR28 | Rust + platform asset hooks | Asset fixtures pass on iOS, Android, and web. | Planned |
| 30 | `codex/rust-supervaldi-30-rust-text-input` | feat: port text rich text and text input | [30-rust-text-input.md](2026-07-07-rust-first-supervaldi/steps/30-rust-text-input.md) | Port text rendering and input behavior to Rust backend. | PR29 | Rust + platform text hooks | Text and text input fixtures pass on iOS, Android, and web. | Planned |
| 31 | `codex/rust-supervaldi-31-rust-accessibility` | feat: port accessibility mapping | [31-rust-accessibility.md](2026-07-07-rust-first-supervaldi/steps/31-rust-accessibility.md) | Port accessibility behavior to Rust backend. | PR30 | Rust + platform accessibility hooks | Accessibility snapshots pass on iOS, Android, and web. | Planned |
| 32 | `codex/rust-supervaldi-32-rust-events-gestures` | feat: port events and gestures | [32-rust-events-gestures.md](2026-07-07-rust-first-supervaldi/steps/32-rust-events-gestures.md) | Port event dispatch and gesture payloads to Rust backend. | PR31 | Rust + Swift/Kotlin/JS event hooks | Event and gesture fixtures reach Rust actions with typed payloads on iOS, Android, and web. | Planned |
| 33 | `codex/rust-supervaldi-33-rust-animations` | feat: port animation dispatch | [33-rust-animations.md](2026-07-07-rust-first-supervaldi/steps/33-rust-animations.md) | Port animation behavior to Rust backend. | PR32 | Rust + platform animation hooks | Animation fixtures start, complete, cancel, and match transaction snapshots on iOS, Android, and web. | Planned |
| 34 | `codex/rust-supervaldi-34-rust-native-view-handoff` | feat: port native view registry and handoff | [34-rust-native-view-handoff.md](2026-07-07-rust-first-supervaldi/steps/34-rust-native-view-handoff.md) | Connect Rust backend rendering to generated native view contracts and platform implementations. | PR33 | Rust + Swift/UIKit + Kotlin/View + JS/DOM + PNG fallback | Native view backend fixtures pass on iOS, Android, web, and PNG fallback. | Planned |
| 35 | `codex/rust-supervaldi-35-rust-backend-samples` | feat: run samples on Rust platform backends | [35-rust-backend-samples.md](2026-07-07-rust-first-supervaldi/steps/35-rust-backend-samples.md) | Exercise Rust platform backends end to end before parity gating. | PR34 | Rust + Swift + Kotlin + JS | Samples run on Rust backend for iOS, Android, and web. | Planned |
| 36 | `codex/rust-supervaldi-36-platform-parity` | test: add full platform parity suite | [36-platform-parity.md](2026-07-07-rust-first-supervaldi/steps/36-platform-parity.md) | Prove retained and Rust backends match across every contract fixture before production release gates. | PR35 | Test infra across Rust, iOS, Android, web, PNG, TS compatibility, and transition targets | Every contract fixture passes declared parity targets. | Planned |
| 37 | `codex/rust-supervaldi-37-release-gates` | ci: add production release gates and budgets | [37-release-gates.md](2026-07-07-rust-first-supervaldi/steps/37-release-gates.md) | Install release-blocking checks before Rust backend becomes the production default. | PR36 | CI + Bazel + Rust tooling + platform test scripts | Named CI gates pass: `rust_forbidden_deps`, `platform_parity`, `module_conformance`, `native_view_conformance`, `png_determinism`, `hot_reload_matrix`, `perf_budgets`. | Planned |
| 38 | `codex/rust-supervaldi-38-production-default` | feat: make Rust backend production default | [38-production-default.md](2026-07-07-rust-first-supervaldi/steps/38-production-default.md) | Make Rust backends the production default for Rust-authored apps. | PR37 | Rust + Bazel + platform app targets | iOS Rust app target excludes TypeScript compiler/runtime, JS direct renderer, TSN, `RenderRequest`, `ViewNodeRenderer`, `ViewNodeTree`, C++ renderer adapter targets, and transition adapter targets. | Planned |
| 39 | `codex/rust-supervaldi-39-production-devex` | chore: add production docs templates and devtools | [39-production-devex.md](2026-07-07-rust-first-supervaldi/steps/39-production-devex.md) | Ship the production developer experience around the Rust-first framework. | PR38 | Docs + templates + CLI wrappers | New Rust app template builds and runs on iOS, Android, and web. | Planned |
| 40 | `codex/rust-supervaldi-40-delete-plan` | cleanup: remove migration plan | [40-delete-plan.md](2026-07-07-rust-first-supervaldi/steps/40-delete-plan.md) | Remove temporary planning documents after production migration is complete. | PR39 | Docs deletion only | Only plan docs are deleted. | Planned |

## Step Details

The implementation detail for each stack step lives under [steps/](2026-07-07-rust-first-supervaldi/steps/). Each file is part of PR00 and records exact changes, code boundary, proof, and plan update for its matching implementation PR.

## Affected Areas

| Area / Feature | Expected Change | Breakage Mode | Affected Users | Validation |
| --- | --- | --- | --- | --- |
| Existing TypeScript apps | Continue on retained compiler, JS runtime, and C++ renderer path | Shared build/runtime change breaks TS sample | Existing Valdi developers | TS sample build/render check in PR10, PR36, PR38 |
| Rust app authoring | Rust DSL and Rust contracts become app surface | DSL cannot express a required platform behavior | Rust app developers | PR09 golden fixtures for every contract row |
| UI IR | Stable production contract across all backends | IR omits a production surface | Framework maintainers | PR01 contract, PR03 schema coverage, PR04 fixtures, PR36 parity |
| Hot reload | Moves from JS source reload to IR patch reload plus declared Rust hot patch class | State resets or unsupported edit applies silently | App developers | PR11/PR12 compatibility and rebuild-required fixtures |
| Native rendering | Rust backend replaces C++ renderer in Rust app path | Rust backend diverges from retained backend | App developers, QA | PR36 parity suite before PR38 default switch |
| Polyglot modules | Rust-authored contracts generate platform factories | Type marshalling mismatch | Product engineers | PR21 cross-target conformance |
| Native views | Rust-authored contracts generate platform stubs and Rust backend handoff | Lifecycle/measurement/event mismatch | Platform teams | PR23 and PR34 conformance |
| Web | Rust DOM backend replaces TS direct-renderer dependency for Rust apps | DOM event/style/text semantics diverge | Web developers | PR16 browser snapshots and PR36 parity |
| PNG | Deterministic renderer consumes same IR | Golden drift hides backend regression | QA/tooling | PR17 and PR25 golden suites |
| Bazel | Rust crates and generated glue enter build graph | Platform build or packaging fails | Build/release engineers | PR02 labels and PR38 forbidden dependency checks |

## Risk Register

| Risk | Affected Feature / Users | Impact | Likelihood | Mitigation | Validation | Owner PR | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Contract misses a production surface | Rust apps and backends | Architecture churn and missing replacement capability | High | PR01 machine-readable contract references repo facts and owner/proof for every surface | PR03, PR04, PR36 consume the contract directly | 01 | Open |
| IR becomes partial despite contract | Runtime/backends | Backend-specific schema drift | High | PR03 fails on any contract row lacking Rust type coverage | Schema coverage test | 03 | Open |
| Hot reload requires full Rust rebuild for common UI edits | Developer workflow | Slow iteration | High | PR11 parses declarative DSL into IR without full Rust compilation for UI edits | Hot reload latency report and fixture edits | 11 | Open |
| Rust logic hot patch claims too much | Developer workflow | Incorrect live code behavior | Medium | PR12 supports only declared action-body edit class and returns rebuild-required for every other class | Support matrix suite | 12 | Open |
| C++ transition leaks into Rust app API | Rust developer ergonomics | Rust apps couple to Valdi internals | High | Transition code lives in adapter crates; PR38 forbidden dependency checks remove it from Rust app graph | Public API and Bazel graph checks | 14,15,38 | Open |
| Module codegen scope undercovers platform behavior | Polyglot modules | Swift/Kotlin/JS/Rust implementations diverge | High | PR18-PR21 split contract, factories, and conformance across the full type matrix | Cross-target conformance | 18-21 | Open |
| Native view contracts hide platform requirements | Native views | Lifecycle, measurement, reuse, event, or accessibility bugs | High | PR22-PR23 own generated stubs; PR34 owns Rust backend handoff | Native view conformance and parity | 22,23,34,36 | Open |
| Rust backend diverges from retained native behavior | Rendering | Layout/event/animation regressions | High | Rust backend lands side-by-side and PR36 blocks production default on parity failures | Full parity suite | 26-38 | Open |
| TS compatibility becomes hidden Rust dependency | Architecture | Rust path remains split-brain | Medium | PR10 keeps TSX-to-IR in compatibility package; PR38 checks Rust app graph | Bazel graph check | 10,38 | Open |

## Roadmap Status

| PR | Status | Required Proof To Advance | Plan Update Required In That PR |
| --- | --- | --- | --- |
| 00 | In progress | Plan PR contains docs only. | Mark PR00 complete after merge. |
| 01 | Complete | Contract check passes. | Added authoritative YAML contract, generated Markdown, and contract checker. |
| 02 | Complete | `bazelisk test` passes for the Rust foundation labels. | Added Rust crate skeleton, Bazel macro layer, crate graph metadata, and foundation tests. |
| 03 | Complete | Schema coverage test proves no contract row lacks a Rust type. | Added full Rust IR schema types, contract-consuming coverage checks, typed extension checks, and public API examples. |
| 04 | Complete | Fixture coverage test proves every contract row and fixture tag pair has coverage. | Added `valdi_rust/fixtures/contract_fixture_manifest.json`, serialized fixture assets, and invalid diagnostic fixture metadata. |
| 05 | Planned | Every fixture round-trips through JSON and binary codecs. | Mark PR05 complete. |
| 06 | Planned | Mock backend snapshots cover every operation family. | Mark PR06 complete. |
| 07 | Planned | Static fixture declarations emit expected ordered backend ops. | Mark PR07 complete. |
| 08 | Planned | State fixtures update predictably. | Mark PR08 complete. |
| 09 | Planned | DSL output equals canonical IR for every contract row. | Mark PR09 complete. |
| 10 | Planned | TSX fixtures produce normalized IR equal to Rust DSL fixtures. | Mark PR10 complete. |
| 11 | Planned | UI tree, style, layout, text, asset, binding, event, accessibility, module ref, and native view ref edits patch live on the sample host without reinstall or relaunch. | Mark PR11 complete. |
| 12 | Planned | Supported action-body edit patches live. | Mark PR12 complete. |
| 13 | Planned | Dynamic producer feeds validator, runtime, and mock backend using the same diagnostics as DSL-produced IR. | Mark PR13 complete. |
| 14 | Planned | iOS simulator renders contract fixtures through retained backend. | Mark PR14 complete. |
| 15 | Planned | Android emulator renders contract fixtures through retained backend. | Mark PR15 complete. |
| 16 | Planned | Browser DOM snapshots pass for DOM-tagged fixtures. | Mark PR16 complete. |
| 17 | Planned | Golden PNGs pass for base static-render fixtures. | Mark PR17 complete. |
| 18 | Planned | Contract type matrix test passes. | Mark PR18 complete. |
| 19 | Planned | Same module conformance passes on iOS and Android. | Mark PR19 complete. |
| 20 | Planned | Same module conformance passes on web, Rust host, C++ transition, and TS compatibility targets. | Mark PR20 complete. |
| 21 | Planned | Complete module conformance passes on iOS, Android, web, Rust host, C++ transition, and TS compatibility. | Mark PR21 complete. |
| 22 | Planned | Native view contract matrix test passes. | Mark PR22 complete. |
| 23 | Planned | Native view stub conformance passes for iOS, Android, web, PNG fallback metadata, and retained compatibility metadata. | Mark PR23 complete. |
| 24 | Planned | Samples build and launch on iOS, Android, and web. | Mark PR24 complete. |
| 25 | Planned | PNG golden suite passes on CI for every static-render fixture. | Mark PR25 complete. |
| 26 | Planned | Rust tree snapshots match retained backend snapshots for tree fixtures. | Mark PR26 complete. |
| 27 | Planned | Layout and attr metrics match retained backend for all layout fixtures. | Mark PR27 complete. |
| 28 | Planned | iOS, Android, and web mutate views from Rust backend transactions. | Mark PR28 complete. |
| 29 | Planned | Asset fixtures pass on iOS, Android, and web. | Mark PR29 complete. |
| 30 | Planned | Text and text input fixtures pass on iOS, Android, and web. | Mark PR30 complete. |
| 31 | Planned | Accessibility snapshots pass on iOS, Android, and web. | Mark PR31 complete. |
| 32 | Planned | Event and gesture fixtures reach Rust actions with typed payloads on iOS, Android, and web. | Mark PR32 complete. |
| 33 | Planned | Animation fixtures start, complete, cancel, and match transaction snapshots on iOS, Android, and web. | Mark PR33 complete. |
| 34 | Planned | Native view backend fixtures pass on iOS, Android, web, and PNG fallback. | Mark PR34 complete. |
| 35 | Planned | Samples run on Rust backend for iOS, Android, and web. | Mark PR35 complete. |
| 36 | Planned | Every contract fixture passes declared parity targets. | Mark PR36 complete. |
| 37 | Planned | Named CI gates pass: `rust_forbidden_deps`, `platform_parity`, `module_conformance`, `native_view_conformance`, `png_determinism`, `hot_reload_matrix`, `perf_budgets`. | Mark PR37 complete. |
| 38 | Planned | iOS Rust app target excludes TypeScript compiler/runtime, JS direct renderer, TSN, `RenderRequest`, `ViewNodeRenderer`, `ViewNodeTree`, C++ renderer adapter targets, and transition adapter targets. | Mark PR38 complete. |
| 39 | Planned | New Rust app template builds and runs on iOS, Android, and web. | Mark PR39 complete. |
| 40 | Planned | Only plan docs are deleted. | No plan row remains because the plan is deleted. |

## Validation Strategy

- Every PR runs formatting and affected tests for touched surfaces.
- Every implementation PR updates this plan's Roadmap Status row, its step file, and Risk Register rows it advances.
- Contract validation: PR01, PR03, PR04, PR05, PR06, PR36, PR37, and PR38 consume the same `replacement_contract.yaml`.
- Local validation uses Bazel targets wherever Bazel targets exist.
- iOS validation uses simulator runs for transition, sample, Rust backend, and parity PRs.
- Android validation uses emulator runs for transition, sample, Rust backend, and parity PRs.
- Web validation uses browser DOM snapshots and screenshots.
- PNG validation uses deterministic golden image tests.
- Build graph validation proves production Rust apps exclude TypeScript compiler/runtime, JS direct renderer, TSN, `RenderRequest`, `ViewNodeRenderer`, `ViewNodeTree`, C++ renderer adapter targets, and transition adapter targets.

## Rollout And Release Notes

- PR00 is plan-only.
- PR01-PR06 establish production contract, Rust foundation, full IR, codecs, validators, and backend operation contract.
- PR07-PR13 establish runtime, state, DSL, TSX compatibility, hot reload, Rust hot patch, and dynamic UI.
- PR14-PR17 prove iOS, Android, web, and PNG rendering before replacing retained native rendering.
- PR18-PR25 complete app-facing module, native view, sample, and PNG product surfaces.
- PR26-PR35 build and exercise the Rust platform backend side-by-side.
- PR36 proves parity for every contract fixture.
- PR37 installs release-blocking CI gates and performance budgets.
- PR38 makes Rust platform backends the production default for Rust-authored apps.
- PR39 adds production templates, docs, and devtools commands.
- PR40 removes this temporary plan and the step detail files.

## Quality Gate Ledger

| PR | Gate | Command Or Review | Result | Proof Artifact |
| --- | --- | --- | --- | --- |
| 00 | Graphite branch | `gt create codex/rust-supervaldi-00-plan -m "plan: Rust-first production migration"` | Branch created | Local branch `codex/rust-supervaldi-00-plan` |
| 00 | Repo research | Reviewed TSX, renderer, RenderRequest, view tree, transaction, attributes, hot reload, TSN, module generator, web, assets, and Bazel paths cited above | Passed | Paths listed in `Current Repo Facts Driving The Plan` |
| 00 | Docs scope | `git diff --cached --stat` and staged file list contain only `docs/plans/2026-07-07-rust-first-supervaldi.md` and `docs/plans/2026-07-07-rust-first-supervaldi/steps/` | Passed | PR00 staged diff |
| 00 | Ambiguity wording scan | `scripts/check_rust_migration_plan_language.sh ambiguity` | Passed | No matches |
| 00 | Missing-surface scan | `scripts/check_rust_migration_plan_language.sh coverage` | Passed | No matches |
| 00 | Simplify review | Reuse/structure, quality/ambiguity, and ordering review agents inspect the plan diff | Passed | Findings fixed before amend |
| 01 | Contract source | `docs/rust_migration/replacement_contract.yaml` | Added | Authoritative replacement contract with 22 surface rows |
| 01 | Generated contract | `docs/rust_migration/replacement_contract.md` | Added | Generated from `replacement_contract.yaml` |
| 01 | Contract check | `python3 scripts/check_rust_migration_contract.py` | Passed | Validates row IDs, owners, proof gates, fixture tags, platform targets, forbidden dependencies, and generated Markdown |
| 02 | Rust foundation tests | `bazelisk test //valdi_rust:foundation_tests` | Passed | 14 executable placeholder and metadata tests passed |
| 02 | Rust crate build | `bazelisk build //valdi_rust/ir:ir //valdi_rust/backend:backend //valdi_rust/runtime:runtime //valdi_rust/codegen:codegen //valdi_rust/cli:cli` | Passed | Existing repo Rust toolchain built all PR02 crate labels |
| 03 | IR schema tests | `bazelisk test //valdi_rust:ir_schema_tests` | Passed | Schema coverage, typed extension, public examples, and contract-consuming tests passed |
| 03 | Rustdoc blocker | `bazelisk test //valdi_rust/ir:ir_doc_tests` | Blocked | `rust_doc_test` generated a runfiles-relative Apple linker path; PR03 proves examples through `//valdi_rust/ir:public_examples_test` instead |
| 04 | Fixture corpus tests | `bazelisk test //valdi_rust:fixture_corpus_tests` | Passed | Contract manifest covers 22 contract rows and 150 row/tag pairs |
| 04 | Fixture crate build | `bazelisk build //valdi_rust/fixtures:fixtures` | Passed | Rust fixture crate compiles static manifest and serialized asset references |
