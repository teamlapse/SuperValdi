import * as fs from 'fs';
import * as path from 'path';
import 'ts-jest';
import {
  ContractFixtureManifest,
  ReplacementContract,
  TSXFeatureCoverageManifest,
  validateTSXFeatureCoverageManifest,
} from './TSXFeatureCoverageManifest';
import { TSX_IR_MANIFEST_DRIFT } from './TSXIRDiagnostics';

function repoRoot(): string {
  let current = process.cwd();
  while (!fs.existsSync(path.join(current, 'docs/rust_migration/replacement_contract.yaml'))) {
    const parent = path.dirname(current);
    if (parent === current) {
      throw new Error('repo root not found');
    }
    current = parent;
  }
  return current;
}

function readJson<T>(repoRelativePath: string): T {
  return JSON.parse(fs.readFileSync(path.join(repoRoot(), repoRelativePath), 'utf8')) as T;
}

describe('TSXFeatureCoverageManifest', () => {
  it('matches the replacement contract and PR04 fixture manifest exactly', () => {
    const manifest = readJson<TSXFeatureCoverageManifest>(
      'compiler/companion/src/tsx_ir_compat/tsx_feature_coverage_manifest.json',
    );
    const contract = readJson<ReplacementContract>('docs/rust_migration/replacement_contract.yaml');
    const fixtureManifest = readJson<ContractFixtureManifest>('valdi_rust/fixtures/contract_fixture_manifest.json');

    expect(validateTSXFeatureCoverageManifest(manifest, contract, fixtureManifest)).toEqual([]);
  });

  it('detects missing TSX row coverage with an exact diagnostic', () => {
    const manifest = readJson<TSXFeatureCoverageManifest>(
      'compiler/companion/src/tsx_ir_compat/tsx_feature_coverage_manifest.json',
    );
    const contract = readJson<ReplacementContract>('docs/rust_migration/replacement_contract.yaml');
    const fixtureManifest = readJson<ContractFixtureManifest>('valdi_rust/fixtures/contract_fixture_manifest.json');
    const drifted = {
      ...manifest,
      fixtures: manifest.fixtures.filter((fixture) => fixture.contract_row_id !== 'build_graph'),
    };

    const diagnostics = validateTSXFeatureCoverageManifest(drifted, contract, fixtureManifest);
    expect(diagnostics[0].code).toBe(TSX_IR_MANIFEST_DRIFT);
    expect(diagnostics[0].path).toBe('$.fixtures.contract_row_id');
  });

  it('detects coverage token drift with an exact diagnostic path', () => {
    const manifest = readJson<TSXFeatureCoverageManifest>(
      'compiler/companion/src/tsx_ir_compat/tsx_feature_coverage_manifest.json',
    );
    const contract = readJson<ReplacementContract>('docs/rust_migration/replacement_contract.yaml');
    const fixtureManifest = readJson<ContractFixtureManifest>('valdi_rust/fixtures/contract_fixture_manifest.json');
    const drifted: TSXFeatureCoverageManifest = {
      ...manifest,
      fixtures: manifest.fixtures.map((fixture) =>
        fixture.contract_row_id === 'ts_compatibility'
          ? { ...fixture, coverage: fixture.coverage.slice(0, -1) }
          : fixture,
      ),
    };

    const diagnostics = validateTSXFeatureCoverageManifest(drifted, contract, fixtureManifest);
    expect(diagnostics.some((diagnostic) => diagnostic.path === '$.fixtures.ts_compatibility.coverage')).toBe(true);
  });
});
