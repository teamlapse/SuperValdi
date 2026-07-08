import {
  ContractRowId,
  REQUIRED_CONTRACT_ROW_IDS,
  fixtureId,
  serializedArtifactPath,
} from './NormalizedIR';
import { TSXIRDiagnostic, TSX_IR_MANIFEST_DRIFT, tsxIRError } from './TSXIRDiagnostics';

export interface ReplacementContractRow {
  readonly id: ContractRowId;
  readonly surface: string;
  readonly coverage: readonly string[];
  readonly fixture_tags: readonly string[];
  readonly platform_targets: readonly string[];
}

export interface ReplacementContract {
  readonly schema_version: 1;
  readonly rows: readonly ReplacementContractRow[];
}

export interface ContractFixtureManifestEntry {
  readonly fixture_id: string;
  readonly contract_row_id: ContractRowId;
  readonly surface: string;
  readonly fixture_tags: readonly string[];
  readonly platform_targets: readonly string[];
  readonly serialized_artifact_path: string;
}

export interface ContractFixtureManifest {
  readonly schema_version: 1;
  readonly fixtures: readonly ContractFixtureManifestEntry[];
}

export interface TSXFeatureCoverageManifestEntry {
  readonly contract_row_id: ContractRowId;
  readonly fixture_id: string;
  readonly surface: string;
  readonly fixture_tags: readonly string[];
  readonly platform_targets: readonly string[];
  readonly coverage: readonly string[];
  readonly serialized_artifact_path: string;
  readonly tsx_fixture_id: string;
  readonly tsx_feature_tags: readonly string[];
  readonly normalized_ir_required_fields: readonly string[];
}

export interface TSXFeatureCoverageManifest {
  readonly schema_version: 1;
  readonly owner_pr: 'PR10';
  readonly source_contract: 'docs/rust_migration/replacement_contract.yaml';
  readonly source_fixture_manifest: 'valdi_rust/fixtures/contract_fixture_manifest.json';
  readonly normalized_ir_emitter: 'tsx_normalized_ir_sidecar_v1';
  readonly direct_renderer_compatibility: 'retained_jsx_processor';
  readonly rust_path_dependency: 'none';
  readonly fixtures: readonly TSXFeatureCoverageManifestEntry[];
}

function listByRowId<T extends { readonly contract_row_id: ContractRowId }>(
  items: readonly T[],
): Map<ContractRowId, T> {
  const byRowId = new Map<ContractRowId, T>();
  for (const item of items) {
    byRowId.set(item.contract_row_id, item);
  }
  return byRowId;
}

function requireEqual(
  actual: unknown,
  expected: unknown,
  path: string,
  diagnostics: TSXIRDiagnostic[],
): void {
  if (JSON.stringify(actual) !== JSON.stringify(expected)) {
    diagnostics.push(
      tsxIRError(TSX_IR_MANIFEST_DRIFT, path, `${path} drift: ${JSON.stringify(actual)} != ${JSON.stringify(expected)}`),
    );
  }
}

export function validateTSXFeatureCoverageManifest(
  manifest: TSXFeatureCoverageManifest,
  contract: ReplacementContract,
  fixtureManifest: ContractFixtureManifest,
): readonly TSXIRDiagnostic[] {
  const diagnostics: TSXIRDiagnostic[] = [];
  requireEqual(manifest.schema_version, 1, '$.schema_version', diagnostics);
  requireEqual(manifest.owner_pr, 'PR10', '$.owner_pr', diagnostics);
  requireEqual(manifest.source_contract, 'docs/rust_migration/replacement_contract.yaml', '$.source_contract', diagnostics);
  requireEqual(
    manifest.source_fixture_manifest,
    'valdi_rust/fixtures/contract_fixture_manifest.json',
    '$.source_fixture_manifest',
    diagnostics,
  );
  requireEqual(manifest.normalized_ir_emitter, 'tsx_normalized_ir_sidecar_v1', '$.normalized_ir_emitter', diagnostics);
  requireEqual(manifest.direct_renderer_compatibility, 'retained_jsx_processor', '$.direct_renderer_compatibility', diagnostics);
  requireEqual(manifest.rust_path_dependency, 'none', '$.rust_path_dependency', diagnostics);

  const contractRowIds = contract.rows.map((row: ReplacementContractRow) => row.id);
  const manifestRowIds = manifest.fixtures.map((fixture: TSXFeatureCoverageManifestEntry) => fixture.contract_row_id);
  requireEqual(contractRowIds, REQUIRED_CONTRACT_ROW_IDS, '$.contract.rows', diagnostics);
  requireEqual(manifestRowIds, REQUIRED_CONTRACT_ROW_IDS, '$.fixtures.contract_row_id', diagnostics);

  const fixturesByRowId = listByRowId(fixtureManifest.fixtures);
  for (const row of contract.rows) {
    const manifestFixture = manifest.fixtures.find(
      (entry: TSXFeatureCoverageManifestEntry) => entry.contract_row_id === row.id,
    );
    const corpusFixture = fixturesByRowId.get(row.id);
    if (!manifestFixture || !corpusFixture) {
      diagnostics.push(tsxIRError(TSX_IR_MANIFEST_DRIFT, `$.fixtures.${row.id}`, `${row.id} is missing`));
      continue;
    }
    requireEqual(manifestFixture.fixture_id, fixtureId(row.id), `$.fixtures.${row.id}.fixture_id`, diagnostics);
    requireEqual(manifestFixture.fixture_id, corpusFixture.fixture_id, `$.fixtures.${row.id}.fixture_id`, diagnostics);
    requireEqual(manifestFixture.surface, row.surface, `$.fixtures.${row.id}.surface`, diagnostics);
    requireEqual(manifestFixture.fixture_tags, row.fixture_tags, `$.fixtures.${row.id}.fixture_tags`, diagnostics);
    requireEqual(manifestFixture.platform_targets, row.platform_targets, `$.fixtures.${row.id}.platform_targets`, diagnostics);
    requireEqual(manifestFixture.coverage, row.coverage, `$.fixtures.${row.id}.coverage`, diagnostics);
    requireEqual(
      manifestFixture.serialized_artifact_path,
      serializedArtifactPath(row.id),
      `$.fixtures.${row.id}.serialized_artifact_path`,
      diagnostics,
    );
    requireEqual(
      manifestFixture.serialized_artifact_path,
      corpusFixture.serialized_artifact_path,
      `$.fixtures.${row.id}.serialized_artifact_path`,
      diagnostics,
    );
  }

  return diagnostics;
}
