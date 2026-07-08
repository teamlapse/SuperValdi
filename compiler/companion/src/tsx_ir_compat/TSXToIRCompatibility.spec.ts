import 'ts-jest';
import {
  REQUIRED_CONTRACT_ROW_IDS,
  canonicalNormalizedIRLine,
  canonicalNormalizedIRSnapshot,
  componentId,
  rootNodeId,
} from './NormalizedIR';
import {
  canonicalTSXNormalizedIRSnapshot,
  diagnosticFromThrown,
  emitAllNormalizedIRFixtures,
  emitNormalizedIRFromTSXSource,
} from './TSXToIRCompatibility';
import { TSX_IR_ATTRIBUTE_MISSING, TSX_IR_CONTRACT_ROW_UNSUPPORTED } from './TSXIRDiagnostics';
import { TSX_IR_FIXTURES, assertFixtureTableCoversRequiredRows } from './TSXIRFixtures';

function captureDiagnostic(fn: () => void) {
  try {
    fn();
  } catch (thrown) {
    return diagnosticFromThrown(thrown);
  }
  throw new Error('expected diagnostic');
}

function expectedDslCanonicalSnapshot(): string {
  const lines = TSX_IR_FIXTURES.map((fixture) =>
    [
      fixture.contractRowId,
      fixture.fixtureId,
      rootNodeId(fixture.contractRowId),
      componentId(fixture.contractRowId),
      fixture.contractRowId,
      fixture.coverage.join(','),
    ].join('|'),
  );
  return `dsl_canonical_ir_v1\n${lines.join('\n')}\n`;
}

describe('TSXToIRCompatibility', () => {
  it('emits deterministic normalized IR equal to the Rust DSL canonical fixture snapshot', () => {
    assertFixtureTableCoversRequiredRows();

    const envelopes = emitAllNormalizedIRFixtures();
    const documents = envelopes.map((envelope) => envelope.document);
    expect(documents.map((document) => document.contractRowId)).toEqual(REQUIRED_CONTRACT_ROW_IDS);

    for (const envelope of envelopes) {
      expect(envelope.schemaVersion).toBe(1);
      expect(envelope.emitter).toBe('tsx_normalized_ir_sidecar_v1');
      expect(envelope.directRendererCompatibility).toEqual({
        retained: true,
        rendererModulePath: 'valdi_core/src/JSX',
      });
      expect(envelope.document.payload.kind).toBe(envelope.document.contractRowId);
      expect(canonicalNormalizedIRLine(envelope.document)).toContain(envelope.document.fixtureId);
    }

    expect(canonicalNormalizedIRSnapshot(documents)).toBe(expectedDslCanonicalSnapshot());
    expect(canonicalTSXNormalizedIRSnapshot()).toBe(expectedDslCanonicalSnapshot());
  });

  it('keeps the fixture source table aligned with every required contract row', () => {
    expect(TSX_IR_FIXTURES).toHaveLength(REQUIRED_CONTRACT_ROW_IDS.length);
    for (const expectedRowId of REQUIRED_CONTRACT_ROW_IDS) {
      const fixture = TSX_IR_FIXTURES.find((entry) => entry.contractRowId === expectedRowId);
      expect(fixture).toBeDefined();
      expect(fixture!.source).toContain(`contractRowId="${expectedRowId}"`);
      expect(fixture!.source).toContain(`fixtureId="contract.${expectedRowId}.v1"`);
      expect(fixture!.source).toContain(`serializedArtifactPath="valdi_rust/fixtures/serialized/${expectedRowId}.ir.json"`);
    }
  });

  it('returns exact diagnostics for unsupported contract rows', () => {
    const error = captureDiagnostic(() => {
      emitNormalizedIRFromTSXSource('<TsxIrFixture contractRowId="unsupported" />', 'bad.tsx');
    });
    expect(error.diagnostic.code).toBe(TSX_IR_CONTRACT_ROW_UNSUPPORTED);
    expect(error.diagnostic.path).toBe('$.tsx.attributes.contractRowId');
    expect(error.diagnostic.severity).toBe('error');
    expect(error.diagnostic.sourceSpan).toEqual({ fileName: 'bad.tsx', line: 1, column: 1 });
  });

  it('returns exact diagnostics for missing fixture metadata', () => {
    const source = '<TsxIrFixture contractRowId="schema_versioning" />';
    const error = captureDiagnostic(() => {
      emitNormalizedIRFromTSXSource(source, 'missing.tsx');
    });
    expect(error.diagnostic.code).toBe(TSX_IR_ATTRIBUTE_MISSING);
    expect(error.diagnostic.path).toBe('$.tsx.attributes.coverage');
    expect(error.diagnostic.severity).toBe('error');
  });
});
