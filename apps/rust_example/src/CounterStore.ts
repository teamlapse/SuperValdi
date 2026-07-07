import { convertBridgeObservableToObservable } from 'bridge_observables/src/utils/convertBridgeObservableToObservable';
import { Observable } from 'valdi_rxjs/src/Observable';

import { count, formatCount, increment, labelBytes, payloadSize } from './RustCounter';

export const count$: Observable<number> = convertBridgeObservableToObservable(count());
export const rustPayloadSize: number = payloadSize(labelBytes('rust'));

export function incrementCount(): void {
  increment();
}

export function formatCountLabel(count: number): string {
  return formatCount('Rust count', count);
}
