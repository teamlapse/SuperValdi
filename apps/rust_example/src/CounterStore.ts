import { Observable } from 'valdi_rxjs/src/Observable';

import { count, describeAfterIncrement, formatCount, formatCountAsync, labelBytes, payloadSize } from './RustCounter';

export const count$: Observable<number> = count();
export const rustPayloadSize: number = payloadSize(labelBytes('rust'));

export function incrementCount(): string {
  return describeAfterIncrement(value => formatCountLabel(value));
}

export function formatCountLabel(count: number): string {
  return formatCount('Rust count', count);
}

export function formatAsyncCountLabel(count: number): Promise<string> {
  return formatCountAsync(count);
}
