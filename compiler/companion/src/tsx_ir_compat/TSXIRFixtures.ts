import {
  ContractRowId,
  FixtureTag,
  PlatformTarget,
  REQUIRED_CONTRACT_ROW_IDS,
  TSXCompatibilityFeatureTag,
  fixtureId,
  serializedArtifactPath,
} from './NormalizedIR';

export interface TSXIRFixtureDefinition {
  readonly contractRowId: ContractRowId;
  readonly fixtureId: string;
  readonly surface: string;
  readonly fixtureTags: readonly FixtureTag[];
  readonly platformTargets: readonly PlatformTarget[];
  readonly coverage: readonly string[];
  readonly serializedArtifactPath: string;
  readonly ownerPRs: readonly string[];
  readonly proofPRs: readonly string[];
  readonly tsxFixtureId: string;
  readonly tsxFeatureTags: readonly TSXCompatibilityFeatureTag[];
  readonly source: string;
}

interface TSXIRFixtureInput {
  readonly contractRowId: ContractRowId;
  readonly surface: string;
  readonly fixtureTags: readonly FixtureTag[];
  readonly platformTargets: readonly PlatformTarget[];
  readonly coverage: readonly string[];
  readonly ownerPRs: readonly string[];
  readonly proofPRs: readonly string[];
}

const COMMON_TSX_FEATURE_TAGS: readonly TSXCompatibilityFeatureTag[] = [
  'tsx_compat',
  'direct_renderer_compat',
  'normalized_ir',
];

function listAttribute(values: readonly string[]): string {
  return values.join('|');
}

function fixtureSource(input: TSXIRFixtureInput): string {
  return [
    '<TsxIrFixture',
    `  contractRowId="${input.contractRowId}"`,
    `  fixtureId="${fixtureId(input.contractRowId)}"`,
    `  surface="${input.surface}"`,
    `  fixtureTags="${listAttribute(input.fixtureTags)}"`,
    `  platformTargets="${listAttribute(input.platformTargets)}"`,
    `  coverage="${listAttribute(input.coverage)}"`,
    `  ownerPRs="${listAttribute(input.ownerPRs)}"`,
    `  proofPRs="${listAttribute(input.proofPRs)}"`,
    `  serializedArtifactPath="${serializedArtifactPath(input.contractRowId)}"`,
    '/>',
  ].join('\n');
}

function defineFixture(input: TSXIRFixtureInput): TSXIRFixtureDefinition {
  return {
    ...input,
    fixtureId: fixtureId(input.contractRowId),
    serializedArtifactPath: serializedArtifactPath(input.contractRowId),
    tsxFixtureId: `tsx.${input.contractRowId}.v1`,
    tsxFeatureTags: COMMON_TSX_FEATURE_TAGS,
    source: fixtureSource(input),
  };
}

export const TSX_IR_FIXTURES: readonly TSXIRFixtureDefinition[] = [
  defineFixture({
    contractRowId: 'schema_versioning',
    surface: 'Schema and versioning',
    fixtureTags: ['ios', 'android', 'web', 'static_png', 'retained_backend', 'rust_backend'],
    platformTargets: ['ios', 'android', 'web', 'png', 'rust_host'],
    coverage: ['schema.version', 'feature flags', 'capability IDs', 'binary/JSON versions'],
    ownerPRs: ['PR01', 'PR03', 'PR05'],
    proofPRs: ['PR05', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'component_identity',
    surface: 'Component identity',
    fixtureTags: ['ios', 'android', 'web', 'retained_backend', 'rust_backend', 'hot_reload', 'dynamic_ui'],
    platformTargets: ['ios', 'android', 'web', 'rust_host'],
    coverage: ['component IDs', 'node IDs', 'keys', 'state identity', 'source spans', 'hot reload identity', 'dynamic identity'],
    ownerPRs: ['PR01', 'PR03', 'PR07', 'PR11', 'PR13'],
    proofPRs: ['PR07', 'PR11', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'tree_structure',
    surface: 'Tree structure',
    fixtureTags: ['ios', 'android', 'web', 'retained_backend', 'rust_backend'],
    platformTargets: ['ios', 'android', 'web', 'rust_host'],
    coverage: ['root', 'parent order', 'keyed movement', 'fragments', 'slots', 'portals', 'contexts', 'destruction', 'pooling'],
    ownerPRs: ['PR01', 'PR03', 'PR06', 'PR26'],
    proofPRs: ['PR26', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'element_taxonomy',
    surface: 'Element taxonomy',
    fixtureTags: ['ios', 'android', 'web', 'dom_snapshot', 'screenshot', 'static_png', 'retained_backend', 'rust_backend', 'native_view'],
    platformTargets: ['ios', 'android', 'web', 'png'],
    coverage: ['view', 'layout', 'scroll', 'image', 'text', 'rich text', 'text input', 'controls', 'list', 'webview', 'native view', 'drawing host'],
    ownerPRs: ['PR01', 'PR03', 'PR04', 'PR16', 'PR26'],
    proofPRs: ['PR04', 'PR16', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'layout',
    surface: 'Layout',
    fixtureTags: ['ios', 'android', 'web', 'dom_snapshot', 'screenshot', 'static_png', 'retained_backend', 'rust_backend'],
    platformTargets: ['ios', 'android', 'web', 'png'],
    coverage: ['Yoga/flex', 'absolute', 'min/max', 'measure', 'safe area', 'scroll sizing', 'z order', 'clipping', 'transform', 'RTL'],
    ownerPRs: ['PR01', 'PR03', 'PR06', 'PR27'],
    proofPRs: ['PR27', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'styling',
    surface: 'Styling',
    fixtureTags: ['ios', 'android', 'web', 'dom_snapshot', 'screenshot', 'static_png', 'retained_backend', 'rust_backend'],
    platformTargets: ['ios', 'android', 'web', 'png'],
    coverage: ['backgrounds', 'colors', 'opacity', 'border', 'radius', 'shadow', 'overflow', 'visibility', 'display', 'transform', 'class metadata', 'platform extensions'],
    ownerPRs: ['PR01', 'PR03', 'PR06', 'PR27'],
    proofPRs: ['PR27', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'text',
    surface: 'Text',
    fixtureTags: ['ios', 'android', 'web', 'dom_snapshot', 'screenshot', 'static_png', 'retained_backend', 'rust_backend'],
    platformTargets: ['ios', 'android', 'web', 'png'],
    coverage: ['plain text', 'rich spans', 'fonts', 'wrapping', 'truncation', 'links', 'measure hooks', 'input value', 'selection', 'composition'],
    ownerPRs: ['PR01', 'PR03', 'PR04', 'PR30'],
    proofPRs: ['PR30', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'assets',
    surface: 'Assets',
    fixtureTags: ['ios', 'android', 'web', 'dom_snapshot', 'screenshot', 'static_png', 'retained_backend', 'rust_backend'],
    platformTargets: ['ios', 'android', 'web', 'png'],
    coverage: ['local assets', 'variants', 'remote refs', 'data refs', 'resize modes', 'loading state', 'errors', 'animated metadata', 'cache identity'],
    ownerPRs: ['PR01', 'PR03', 'PR04', 'PR29'],
    proofPRs: ['PR29', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'events_gestures',
    surface: 'Events and gestures',
    fixtureTags: ['ios', 'android', 'web', 'retained_backend', 'rust_backend', 'hot_reload'],
    platformTargets: ['ios', 'android', 'web', 'rust_host'],
    coverage: ['tap', 'press', 'long press', 'pan', 'scroll', 'focus', 'blur', 'input', 'keyboard submit', 'layout', 'draw', 'visibility', 'frame observer', 'custom native events'],
    ownerPRs: ['PR01', 'PR03', 'PR08', 'PR32'],
    proofPRs: ['PR32', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'actions_state',
    surface: 'Actions and state',
    fixtureTags: ['ios', 'android', 'web', 'retained_backend', 'rust_backend', 'hot_reload', 'dynamic_ui'],
    platformTargets: ['ios', 'android', 'web', 'rust_host'],
    coverage: ['sync actions', 'async actions', 'results', 'typed errors', 'cancellation', 'invalidation', 'scheduling', 'coalescing'],
    ownerPRs: ['PR01', 'PR03', 'PR08', 'PR11', 'PR12'],
    proofPRs: ['PR08', 'PR12', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'bindings_expressions',
    surface: 'Bindings and expressions',
    fixtureTags: ['ios', 'android', 'web', 'retained_backend', 'rust_backend', 'hot_reload', 'dynamic_ui'],
    platformTargets: ['ios', 'android', 'web', 'rust_host'],
    coverage: ['field paths', 'literals', 'nullable values', 'logic', 'comparisons', 'lists', 'computed projections', 'platform constants', 'source spans'],
    ownerPRs: ['PR01', 'PR03', 'PR08', 'PR09', 'PR11'],
    proofPRs: ['PR08', 'PR09', 'PR11', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'animations',
    surface: 'Animations',
    fixtureTags: ['ios', 'android', 'web', 'screenshot', 'retained_backend', 'rust_backend'],
    platformTargets: ['ios', 'android', 'web', 'rust_host'],
    coverage: ['start', 'end', 'cancel', 'timing', 'easing', 'delay', 'repeat', 'fill mode', 'property animation', 'layout animation', 'transaction grouping'],
    ownerPRs: ['PR01', 'PR03', 'PR06', 'PR33'],
    proofPRs: ['PR33', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'native_modules',
    surface: 'Native modules',
    fixtureTags: ['ios', 'android', 'web', 'module', 'retained_backend', 'rust_backend', 'tsx_compat'],
    platformTargets: ['ios', 'android', 'web', 'rust_host', 'swift', 'kotlin', 'js_dom', 'cpp_transition', 'ts_compatibility'],
    coverage: ['Rust contracts', 'complete type matrix', 'dispatch targets', 'Swift/Kotlin/JS/Rust/C++ transition/TS factories'],
    ownerPRs: ['PR01', 'PR18', 'PR19', 'PR20', 'PR21'],
    proofPRs: ['PR21', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'native_views',
    surface: 'Native views',
    fixtureTags: ['ios', 'android', 'web', 'native_view', 'screenshot', 'static_png', 'retained_backend', 'rust_backend'],
    platformTargets: ['ios', 'android', 'web', 'png', 'swift', 'kotlin', 'js_dom', 'cpp_transition', 'ts_compatibility'],
    coverage: ['Rust contracts', 'typed attrs', 'typed events', 'lifecycle', 'measurement', 'reuse', 'platform extensions', 'accessibility', 'fallback'],
    ownerPRs: ['PR01', 'PR22', 'PR23', 'PR34'],
    proofPRs: ['PR23', 'PR34', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'accessibility',
    surface: 'Accessibility',
    fixtureTags: ['ios', 'android', 'web', 'dom_snapshot', 'screenshot', 'static_png', 'retained_backend', 'rust_backend'],
    platformTargets: ['ios', 'android', 'web', 'png'],
    coverage: ['role', 'label', 'hint', 'value', 'state', 'action', 'focus order', 'grouping', 'hidden state', 'platform overrides'],
    ownerPRs: ['PR01', 'PR03', 'PR31'],
    proofPRs: ['PR31', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'hot_reload',
    surface: 'Hot reload',
    fixtureTags: ['ios', 'android', 'web', 'hot_reload', 'retained_backend', 'rust_backend', 'module', 'native_view', 'dynamic_ui'],
    platformTargets: ['ios', 'android', 'web', 'rust_host'],
    coverage: ['patch identity', 'source spans', 'compatibility', 'assets', 'bindings', 'actions', 'module refs', 'native view refs'],
    ownerPRs: ['PR01', 'PR11', 'PR12'],
    proofPRs: ['PR11', 'PR12', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'diagnostics',
    surface: 'Diagnostics',
    fixtureTags: ['ios', 'android', 'web', 'static_png', 'module', 'native_view', 'hot_reload', 'dynamic_ui', 'tsx_compat'],
    platformTargets: ['ios', 'android', 'web', 'png', 'rust_host', 'ts_compatibility'],
    coverage: ['schema paths', 'source spans', 'backend paths', 'capability errors', 'unsupported attrs/events', 'fixture IDs', 'action IDs', 'module IDs'],
    ownerPRs: ['PR01', 'PR05', 'PR06', 'PR27'],
    proofPRs: ['PR05', 'PR27', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'web_dom',
    surface: 'Web DOM',
    fixtureTags: ['web', 'dom_snapshot', 'screenshot', 'rust_backend'],
    platformTargets: ['web', 'js_dom'],
    coverage: ['DOM mapping', 'CSS/style mapping', 'events', 'text measurement', 'class emission', 'browser snapshots'],
    ownerPRs: ['PR01', 'PR16', 'PR28'],
    proofPRs: ['PR16', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'png_backend',
    surface: 'PNG backend',
    fixtureTags: ['static_png', 'screenshot', 'native_view'],
    platformTargets: ['png'],
    coverage: ['layout snapshot', 'display list', 'text', 'image', 'native/webview fallback', 'accessibility debug metadata'],
    ownerPRs: ['PR01', 'PR17', 'PR25'],
    proofPRs: ['PR17', 'PR25', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'dynamic_ui',
    surface: 'Dynamic UI',
    fixtureTags: ['dynamic_ui', 'ios', 'android', 'web', 'retained_backend', 'rust_backend'],
    platformTargets: ['ios', 'android', 'web', 'rust_host'],
    coverage: ['validated IR from Rust DSL', 'generated fixtures', 'JSON debug', 'binary bytes', 'in-memory producers'],
    ownerPRs: ['PR01', 'PR05', 'PR13'],
    proofPRs: ['PR13', 'PR36'],
  }),
  defineFixture({
    contractRowId: 'ts_compatibility',
    surface: 'TS compatibility',
    fixtureTags: ['tsx_compat', 'ios', 'android', 'web', 'retained_backend'],
    platformTargets: ['ios', 'android', 'web', 'ts_compatibility'],
    coverage: ['TSX coverage map', 'direct renderer compatibility', 'TSX-to-IR equivalence', 'no Rust-path TS dependency'],
    ownerPRs: ['PR01', 'PR10', 'PR36', 'PR38'],
    proofPRs: ['PR10', 'PR36', 'PR38'],
  }),
  defineFixture({
    contractRowId: 'build_graph',
    surface: 'Build graph',
    fixtureTags: ['ios', 'android', 'web', 'module', 'native_view', 'tsx_compat'],
    platformTargets: ['ios', 'android', 'web', 'rust_host', 'cpp_transition', 'ts_compatibility'],
    coverage: ['Bazel targets', 'crate graph', 'generated glue', 'platform app targets', 'compatibility labels', 'forbidden dependency checks'],
    ownerPRs: ['PR01', 'PR02', 'PR14', 'PR15', 'PR16', 'PR18', 'PR19', 'PR20', 'PR22', 'PR23', 'PR37', 'PR38'],
    proofPRs: ['PR02', 'PR37', 'PR38'],
  }),
];

export function fixtureByContractRowId(contractRowId: ContractRowId): TSXIRFixtureDefinition {
  const fixture = TSX_IR_FIXTURES.find((entry: TSXIRFixtureDefinition) => entry.contractRowId === contractRowId);
  if (!fixture) {
    throw new Error(`missing TSX IR fixture for ${contractRowId}`);
  }
  return fixture;
}

export function assertFixtureTableCoversRequiredRows(): void {
  const rows = TSX_IR_FIXTURES.map((fixture: TSXIRFixtureDefinition) => fixture.contractRowId);
  if (rows.join('|') !== REQUIRED_CONTRACT_ROW_IDS.join('|')) {
    throw new Error(`TSX IR fixture row drift: ${rows.join(',')}`);
  }
}
