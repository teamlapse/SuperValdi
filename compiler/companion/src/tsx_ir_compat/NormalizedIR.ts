export const NORMALIZED_IR_SCHEMA_VERSION = 1;
export const TSX_NORMALIZED_IR_EMITTER = 'tsx_normalized_ir_sidecar_v1';
export const RETAINED_DIRECT_RENDERER_MODULE = 'valdi_core/src/JSX';

export const REQUIRED_CONTRACT_ROW_IDS = [
  'schema_versioning',
  'component_identity',
  'tree_structure',
  'element_taxonomy',
  'layout',
  'styling',
  'text',
  'assets',
  'events_gestures',
  'actions_state',
  'bindings_expressions',
  'animations',
  'native_modules',
  'native_views',
  'accessibility',
  'hot_reload',
  'diagnostics',
  'web_dom',
  'png_backend',
  'dynamic_ui',
  'ts_compatibility',
  'build_graph',
] as const;

export type ContractRowId = (typeof REQUIRED_CONTRACT_ROW_IDS)[number];

export type FixtureTag =
  | 'ios'
  | 'android'
  | 'web'
  | 'dom_snapshot'
  | 'screenshot'
  | 'static_png'
  | 'retained_backend'
  | 'rust_backend'
  | 'module'
  | 'native_view'
  | 'hot_reload'
  | 'dynamic_ui'
  | 'tsx_compat';

export type PlatformTarget =
  | 'ios'
  | 'android'
  | 'web'
  | 'png'
  | 'rust_host'
  | 'swift'
  | 'kotlin'
  | 'js_dom'
  | 'cpp_transition'
  | 'ts_compatibility';

export type TSXCompatibilityFeatureTag = 'tsx_compat' | 'direct_renderer_compat' | 'normalized_ir';

export interface NormalizedIRMetadata {
  readonly ownerPRs: readonly string[];
  readonly proofPRs: readonly string[];
  readonly serializedArtifactPath: string;
}

export interface NormalizedIRDebug {
  readonly contractSurface: ContractRowId;
  readonly rootNodeId: string;
  readonly componentId: string;
  readonly coverageTokens: readonly string[];
}

export interface NormalizedIRDocument {
  readonly fixtureId: string;
  readonly contractRowId: ContractRowId;
  readonly surface: string;
  readonly fixtureTags: readonly FixtureTag[];
  readonly platformTargets: readonly PlatformTarget[];
  readonly metadata: NormalizedIRMetadata;
  readonly debug: NormalizedIRDebug;
  readonly payload: NormalizedSurfacePayload;
}

export interface NormalizedIREnvelope {
  readonly schemaVersion: typeof NORMALIZED_IR_SCHEMA_VERSION;
  readonly emitter: typeof TSX_NORMALIZED_IR_EMITTER;
  readonly source: {
    readonly kind: 'tsx_fixture';
    readonly fixtureId: string;
    readonly fileName: string;
  };
  readonly directRendererCompatibility: {
    readonly retained: true;
    readonly rendererModulePath: typeof RETAINED_DIRECT_RENDERER_MODULE;
  };
  readonly document: NormalizedIRDocument;
}

export type NormalizedSurfacePayload =
  | { readonly kind: 'schema_versioning'; readonly schema: { readonly documentVersion: 1; readonly wireFormats: readonly string[] } }
  | { readonly kind: 'component_identity'; readonly identity: { readonly componentId: string; readonly rootNodeId: string; readonly stateScope: string } }
  | { readonly kind: 'tree_structure'; readonly tree: { readonly rootNodeId: string; readonly childOrdering: 'stable'; readonly supportsPortals: true } }
  | { readonly kind: 'element_taxonomy'; readonly elements: readonly string[] }
  | { readonly kind: 'layout'; readonly layout: { readonly engine: 'yoga_compatible'; readonly supportsRtl: true; readonly supportsMeasurement: true } }
  | { readonly kind: 'styling'; readonly style: { readonly classMetadata: true; readonly platformExtensions: 'typed'; readonly properties: readonly string[] } }
  | { readonly kind: 'text'; readonly text: { readonly richText: true; readonly inputState: true; readonly measurementHook: true } }
  | { readonly kind: 'assets'; readonly assets: { readonly local: true; readonly remote: true; readonly variants: true; readonly cacheIdentity: true } }
  | { readonly kind: 'events_gestures'; readonly events: { readonly gestures: readonly string[]; readonly observerEvents: readonly string[] } }
  | { readonly kind: 'actions_state'; readonly actions: { readonly scheduling: true; readonly cancellation: true; readonly typedResults: true } }
  | { readonly kind: 'bindings_expressions'; readonly bindings: { readonly expressionFamilies: readonly string[]; readonly sourceSpans: true } }
  | { readonly kind: 'animations'; readonly animations: { readonly lifecycle: readonly string[]; readonly transactionGrouping: true } }
  | { readonly kind: 'native_modules'; readonly nativeModules: { readonly factoryTargets: readonly string[]; readonly typeMatrix: 'complete' } }
  | { readonly kind: 'native_views'; readonly nativeViews: { readonly lifecycle: true; readonly measurement: true; readonly fallback: true } }
  | { readonly kind: 'accessibility'; readonly accessibility: { readonly role: true; readonly state: true; readonly platformOverrides: true } }
  | { readonly kind: 'hot_reload'; readonly hotReload: { readonly patchIdentity: true; readonly moduleRefs: true; readonly nativeViewRefs: true } }
  | { readonly kind: 'diagnostics'; readonly diagnostics: { readonly stableCodes: true; readonly schemaPaths: true; readonly sourceSpans: true } }
  | { readonly kind: 'web_dom'; readonly webDom: { readonly domMapping: true; readonly cssMapping: true; readonly browserSnapshots: true } }
  | { readonly kind: 'png_backend'; readonly png: { readonly displayList: true; readonly fallbackMetadata: true; readonly deterministic: true } }
  | { readonly kind: 'dynamic_ui'; readonly dynamicUi: { readonly debugJson: true; readonly binaryBytes: true; readonly inMemoryProducers: true } }
  | { readonly kind: 'ts_compatibility'; readonly tsCompatibility: { readonly coverageMap: true; readonly directRendererCompatibility: true; readonly rustPathDependencyCheck: true } }
  | { readonly kind: 'build_graph'; readonly buildGraph: { readonly bazelTargets: true; readonly crateGraph: true; readonly forbiddenDependencyChecks: true } };

export function isContractRowId(value: string): value is ContractRowId {
  return (REQUIRED_CONTRACT_ROW_IDS as readonly string[]).includes(value);
}

export function rootNodeId(contractRowId: ContractRowId): string {
  return `node.${contractRowId}.root`;
}

export function componentId(contractRowId: ContractRowId): string {
  return `component.${contractRowId}`;
}

export function fixtureId(contractRowId: ContractRowId): string {
  return `contract.${contractRowId}.v1`;
}

export function serializedArtifactPath(contractRowId: ContractRowId): string {
  return `valdi_rust/fixtures/serialized/${contractRowId}.ir.json`;
}

export function buildSurfacePayload(
  contractRowId: ContractRowId,
  coverageTokens: readonly string[],
): NormalizedSurfacePayload {
  switch (contractRowId) {
    case 'schema_versioning':
      return { kind: contractRowId, schema: { documentVersion: 1, wireFormats: ['json_debug', 'postcard_binary'] } };
    case 'component_identity':
      return { kind: contractRowId, identity: { componentId: componentId(contractRowId), rootNodeId: rootNodeId(contractRowId), stateScope: 'stable_state_slots' } };
    case 'tree_structure':
      return { kind: contractRowId, tree: { rootNodeId: rootNodeId(contractRowId), childOrdering: 'stable', supportsPortals: true } };
    case 'element_taxonomy':
      return { kind: contractRowId, elements: coverageTokens };
    case 'layout':
      return { kind: contractRowId, layout: { engine: 'yoga_compatible', supportsRtl: true, supportsMeasurement: true } };
    case 'styling':
      return { kind: contractRowId, style: { classMetadata: true, platformExtensions: 'typed', properties: coverageTokens } };
    case 'text':
      return { kind: contractRowId, text: { richText: true, inputState: true, measurementHook: true } };
    case 'assets':
      return { kind: contractRowId, assets: { local: true, remote: true, variants: true, cacheIdentity: true } };
    case 'events_gestures':
      return { kind: contractRowId, events: { gestures: ['tap', 'press', 'long_press', 'pan'], observerEvents: ['layout', 'draw', 'visibility', 'frame'] } };
    case 'actions_state':
      return { kind: contractRowId, actions: { scheduling: true, cancellation: true, typedResults: true } };
    case 'bindings_expressions':
      return { kind: contractRowId, bindings: { expressionFamilies: coverageTokens, sourceSpans: true } };
    case 'animations':
      return { kind: contractRowId, animations: { lifecycle: ['start', 'end', 'cancel'], transactionGrouping: true } };
    case 'native_modules':
      return { kind: contractRowId, nativeModules: { factoryTargets: ['swift', 'kotlin', 'js_dom', 'rust_host', 'cpp_transition', 'ts_compatibility'], typeMatrix: 'complete' } };
    case 'native_views':
      return { kind: contractRowId, nativeViews: { lifecycle: true, measurement: true, fallback: true } };
    case 'accessibility':
      return { kind: contractRowId, accessibility: { role: true, state: true, platformOverrides: true } };
    case 'hot_reload':
      return { kind: contractRowId, hotReload: { patchIdentity: true, moduleRefs: true, nativeViewRefs: true } };
    case 'diagnostics':
      return { kind: contractRowId, diagnostics: { stableCodes: true, schemaPaths: true, sourceSpans: true } };
    case 'web_dom':
      return { kind: contractRowId, webDom: { domMapping: true, cssMapping: true, browserSnapshots: true } };
    case 'png_backend':
      return { kind: contractRowId, png: { displayList: true, fallbackMetadata: true, deterministic: true } };
    case 'dynamic_ui':
      return { kind: contractRowId, dynamicUi: { debugJson: true, binaryBytes: true, inMemoryProducers: true } };
    case 'ts_compatibility':
      return { kind: contractRowId, tsCompatibility: { coverageMap: true, directRendererCompatibility: true, rustPathDependencyCheck: true } };
    case 'build_graph':
      return { kind: contractRowId, buildGraph: { bazelTargets: true, crateGraph: true, forbiddenDependencyChecks: true } };
  }
}

export function canonicalNormalizedIRLine(document: NormalizedIRDocument): string {
  return [
    document.contractRowId,
    document.fixtureId,
    document.debug.rootNodeId,
    document.debug.componentId,
    document.payload.kind,
    document.debug.coverageTokens.join(','),
  ].join('|');
}

export function canonicalNormalizedIRSnapshot(documents: readonly NormalizedIRDocument[]): string {
  const lines = ['dsl_canonical_ir_v1', ...documents.map(canonicalNormalizedIRLine)];
  return `${lines.join('\n')}\n`;
}
