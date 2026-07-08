import * as ts from 'typescript';
import {
  ContractRowId,
  FixtureTag,
  NORMALIZED_IR_SCHEMA_VERSION,
  NormalizedIREnvelope,
  NormalizedIRDocument,
  PlatformTarget,
  RETAINED_DIRECT_RENDERER_MODULE,
  TSX_NORMALIZED_IR_EMITTER,
  buildSurfacePayload,
  canonicalNormalizedIRSnapshot,
  componentId,
  fixtureId,
  isContractRowId,
  rootNodeId,
  serializedArtifactPath,
} from './NormalizedIR';
import {
  TSXIRDiagnosticError,
  TSX_IR_ATTRIBUTE_MISSING,
  TSX_IR_ATTRIBUTE_UNSUPPORTED,
  TSX_IR_CONTRACT_ROW_UNSUPPORTED,
  throwTSXIRDiagnostic,
  tsxIRError,
} from './TSXIRDiagnostics';
import { TSXIRFixtureDefinition, TSX_IR_FIXTURES } from './TSXIRFixtures';

type AttributeMap = Map<string, string>;

const FIXTURE_COMPONENT_NAME = 'TsxIrFixture';

function sourceSpan(sourceFile: ts.SourceFile, node: ts.Node) {
  const position = sourceFile.getLineAndCharacterOfPosition(node.getStart(sourceFile));
  return {
    fileName: sourceFile.fileName,
    line: position.line + 1,
    column: position.character + 1,
  };
}

function readStaticStringAttribute(sourceFile: ts.SourceFile, attribute: ts.JsxAttribute): string {
  const initializer = attribute.initializer;
  if (!initializer) {
    return 'true';
  }
  if (ts.isStringLiteral(initializer)) {
    return initializer.text;
  }
  if (ts.isJsxExpression(initializer) && initializer.expression) {
    if (ts.isStringLiteral(initializer.expression) || ts.isNoSubstitutionTemplateLiteral(initializer.expression)) {
      return initializer.expression.text;
    }
  }
  return throwTSXIRDiagnostic(
    tsxIRError(
      TSX_IR_ATTRIBUTE_UNSUPPORTED,
      `$.tsx.attributes.${attribute.name.getText(sourceFile)}`,
      'TSX IR fixture attributes must be static string values',
      sourceSpan(sourceFile, attribute),
    ),
  );
}

function findFixtureElement(sourceFile: ts.SourceFile): ts.JsxOpeningLikeElement {
  let fixtureElement: ts.JsxOpeningLikeElement | undefined;

  function visit(node: ts.Node): void {
    if (
      (ts.isJsxOpeningElement(node) || ts.isJsxSelfClosingElement(node)) &&
      node.tagName.getText(sourceFile) === FIXTURE_COMPONENT_NAME
    ) {
      fixtureElement = node;
      return;
    }
    ts.forEachChild(node, visit);
  }

  visit(sourceFile);
  if (!fixtureElement) {
    return throwTSXIRDiagnostic(
      tsxIRError(
        TSX_IR_ATTRIBUTE_MISSING,
        '$.tsx.fixture',
        `TSX IR fixture must contain <${FIXTURE_COMPONENT_NAME} />`,
      ),
    );
  }
  return fixtureElement;
}

function collectAttributes(sourceFile: ts.SourceFile, element: ts.JsxOpeningLikeElement): AttributeMap {
  const attributes: AttributeMap = new Map();
  for (const property of element.attributes.properties) {
    if (!ts.isJsxAttribute(property)) {
      return throwTSXIRDiagnostic(
        tsxIRError(
          TSX_IR_ATTRIBUTE_UNSUPPORTED,
          '$.tsx.attributes.spread',
          'TSX IR fixture metadata does not allow spread attributes',
          sourceSpan(sourceFile, property),
        ),
      );
    }
    attributes.set(property.name.getText(sourceFile), readStaticStringAttribute(sourceFile, property));
  }
  return attributes;
}

function requireAttribute(
  attributes: AttributeMap,
  name: string,
  sourceFile: ts.SourceFile,
  element: ts.JsxOpeningLikeElement,
): string {
  const value = attributes.get(name);
  if (!value || value.trim().length === 0) {
    return throwTSXIRDiagnostic(
      tsxIRError(
        TSX_IR_ATTRIBUTE_MISSING,
        `$.tsx.attributes.${name}`,
        `${name} is required`,
        sourceSpan(sourceFile, element),
      ),
    );
  }
  return value;
}

function listAttribute(value: string): readonly string[] {
  if (!value) {
    return [];
  }
  return value.split('|').map((entry: string) => entry.trim());
}

export function emitNormalizedIRFromTSXSource(source: string, fileName: string): NormalizedIREnvelope {
  const sourceFile = ts.createSourceFile(fileName, source, ts.ScriptTarget.Latest, true, ts.ScriptKind.TSX);
  const element = findFixtureElement(sourceFile);
  const attributes = collectAttributes(sourceFile, element);
  const rowId = requireAttribute(attributes, 'contractRowId', sourceFile, element);
  if (!isContractRowId(rowId)) {
    return throwTSXIRDiagnostic(
      tsxIRError(
        TSX_IR_CONTRACT_ROW_UNSUPPORTED,
        '$.tsx.attributes.contractRowId',
        `unsupported contract row ${rowId}`,
        sourceSpan(sourceFile, element),
      ),
    );
  }

  const document = documentFromAttributes(rowId, attributes, sourceFile, element);
  return {
    schemaVersion: NORMALIZED_IR_SCHEMA_VERSION,
    emitter: TSX_NORMALIZED_IR_EMITTER,
    source: {
      kind: 'tsx_fixture',
      fixtureId: document.fixtureId,
      fileName,
    },
    directRendererCompatibility: {
      retained: true,
      rendererModulePath: RETAINED_DIRECT_RENDERER_MODULE,
    },
    document,
  };
}

function requireExactAttribute(
  attributes: AttributeMap,
  name: string,
  expected: string,
  sourceFile: ts.SourceFile,
  element: ts.JsxOpeningLikeElement,
): string {
  const actual = requireAttribute(attributes, name, sourceFile, element);
  if (actual !== expected) {
    return throwTSXIRDiagnostic(
      tsxIRError(
        TSX_IR_ATTRIBUTE_UNSUPPORTED,
        `$.tsx.attributes.${name}`,
        `${name} drift: ${actual} != ${expected}`,
        sourceSpan(sourceFile, element),
      ),
    );
  }
  return actual;
}

function documentFromAttributes(
  contractRowId: ContractRowId,
  attributes: AttributeMap,
  sourceFile: ts.SourceFile,
  element: ts.JsxOpeningLikeElement,
): NormalizedIRDocument {
  const coverageTokens = listAttribute(requireAttribute(attributes, 'coverage', sourceFile, element));
  return {
    fixtureId: requireExactAttribute(attributes, 'fixtureId', fixtureId(contractRowId), sourceFile, element),
    contractRowId,
    surface: requireAttribute(attributes, 'surface', sourceFile, element),
    fixtureTags: listAttribute(requireAttribute(attributes, 'fixtureTags', sourceFile, element)) as readonly FixtureTag[],
    platformTargets: listAttribute(
      requireAttribute(attributes, 'platformTargets', sourceFile, element),
    ) as readonly PlatformTarget[],
    metadata: {
      ownerPRs: listAttribute(requireAttribute(attributes, 'ownerPRs', sourceFile, element)),
      proofPRs: listAttribute(requireAttribute(attributes, 'proofPRs', sourceFile, element)),
      serializedArtifactPath: requireExactAttribute(
        attributes,
        'serializedArtifactPath',
        serializedArtifactPath(contractRowId),
        sourceFile,
        element,
      ),
    },
    debug: {
      contractSurface: contractRowId,
      rootNodeId: rootNodeId(contractRowId),
      componentId: componentId(contractRowId),
      coverageTokens,
    },
    payload: buildSurfacePayload(contractRowId, coverageTokens),
  };
}

export function emitNormalizedIRForFixture(fixture: TSXIRFixtureDefinition): NormalizedIREnvelope {
  return emitNormalizedIRFromTSXSource(fixture.source, `${fixture.tsxFixtureId}.tsx`);
}

export function emitAllNormalizedIRFixtures(): readonly NormalizedIREnvelope[] {
  return TSX_IR_FIXTURES.map(emitNormalizedIRForFixture);
}

export function canonicalTSXNormalizedIRSnapshot(): string {
  return canonicalNormalizedIRSnapshot(
    emitAllNormalizedIRFixtures().map((envelope: NormalizedIREnvelope) => envelope.document),
  );
}

export function diagnosticFromThrown(error: unknown): TSXIRDiagnosticError {
  if (error instanceof TSXIRDiagnosticError) {
    return error;
  }
  throw error;
}
