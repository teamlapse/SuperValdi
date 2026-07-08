export type TSXIRDiagnosticSeverity = 'error' | 'warning';

export interface TSXIRSourceSpan {
  readonly fileName: string;
  readonly line: number;
  readonly column: number;
}

export interface TSXIRDiagnostic {
  readonly code: string;
  readonly path: string;
  readonly severity: TSXIRDiagnosticSeverity;
  readonly message: string;
  readonly sourceSpan?: TSXIRSourceSpan;
}

export const TSX_IR_ATTRIBUTE_MISSING = 'TSX_IR_ATTRIBUTE_MISSING';
export const TSX_IR_ATTRIBUTE_UNSUPPORTED = 'TSX_IR_ATTRIBUTE_UNSUPPORTED';
export const TSX_IR_CONTRACT_ROW_UNSUPPORTED = 'TSX_IR_CONTRACT_ROW_UNSUPPORTED';
export const TSX_IR_COVERAGE_DRIFT = 'TSX_IR_COVERAGE_DRIFT';
export const TSX_IR_MANIFEST_DRIFT = 'TSX_IR_MANIFEST_DRIFT';
export const TSX_IR_FIXTURE_NOT_FOUND = 'TSX_IR_FIXTURE_NOT_FOUND';

export class TSXIRDiagnosticError extends Error {
  constructor(readonly diagnostic: TSXIRDiagnostic) {
    super(`${diagnostic.code} at ${diagnostic.path}: ${diagnostic.message}`);
  }
}

export function tsxIRError(
  code: string,
  path: string,
  message: string,
  sourceSpan?: TSXIRSourceSpan,
): TSXIRDiagnostic {
  return {
    code,
    path,
    severity: 'error',
    message,
    sourceSpan,
  };
}

export function throwTSXIRDiagnostic(diagnostic: TSXIRDiagnostic): never {
  throw new TSXIRDiagnosticError(diagnostic);
}
