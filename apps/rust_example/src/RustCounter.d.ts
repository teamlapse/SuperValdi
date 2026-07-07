import { BridgeObservable } from 'bridge_observables/src/types/BridgeObservable';

/**
 * @ExportModule
 */

// @ExportFunction
export function count(): BridgeObservable<number>;

// @ExportFunction
export function increment(): void;

// @ExportFunction
export function formatCount(prefix: string, value: number): string;

// @ExportFunction
export function labelBytes(label: string): Uint8Array;

// @ExportFunction
export function payloadSize(payload: Uint8Array): number;
