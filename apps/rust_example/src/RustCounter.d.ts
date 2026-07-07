import { Observable } from 'valdi_rxjs/src/Observable';

/**
 * @ExportModule
 */

// @ExportModel
export interface CounterPayload {
  label: string;
  value: number;
}

// @ExportFunction
export function count(): Observable<number>;

// @ExportFunction
export function increment(): void;

// @ExportFunction
export function describeAfterIncrement(formatter: (value: number) => string): string;

// @ExportFunction
export function formatCount(prefix: string, value: number): string;

// @ExportFunction
export function formatCountAsync(value: number): Promise<string>;

// @ExportFunction
export function labelBytes(label: string): Uint8Array;

// @ExportFunction
export function payloadSize(payload: Uint8Array): number;

// @ExportFunction
export function echoPayload(payload: CounterPayload): CounterPayload;
