import { Observable } from 'valdi_rxjs/src/Observable';

import { count, describeAfterIncrement, echoPayload, formatCount, formatCountAsync, labelBytes, payloadSize } from './RustCounter';

export const count$: Observable<number> = count();
export const rustPayloadSize: number = payloadSize(labelBytes('rust'));
const echoedPayload = echoPayload({ label: 'rust payload', value: 7 });
export const rustPayloadSummary: string = `${echoedPayload.label}: ${echoedPayload.value}`;

export function incrementCount(): string {
  return describeAfterIncrement(value => formatCountLabel(value));
}

export function formatCountLabel(count: number): string {
  return formatCount('Rust count', count);
}

export function formatAsyncCountLabel(count: number): Promise<string> {
  return formatCountAsync(count);
}
