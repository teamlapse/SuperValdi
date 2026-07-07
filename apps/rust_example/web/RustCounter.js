let countValue = 0;
let wasmExports = null;
let wasmLoadStarted = false;
const observers = [];

function maybeLoadWasm() {
  if (wasmLoadStarted || typeof WebAssembly === 'undefined') {
    return;
  }
  wasmLoadStarted = true;

  if (typeof process !== 'undefined' && process.versions && process.versions.node) {
    try {
      const fs = require('fs');
      const path = require('path');
      const bytes = fs.readFileSync(path.join(__dirname, 'rust_counter_web_wasm.wasm'));
      const module = new WebAssembly.Module(bytes);
      wasmExports = new WebAssembly.Instance(module, {}).exports;
      return;
    } catch (_error) {
      return;
    }
  }

  if (typeof fetch === 'function') {
    const baseUrl =
      typeof document !== 'undefined' && document.currentScript && document.currentScript.src
        ? document.currentScript.src
        : typeof location !== 'undefined'
          ? location.href
          : undefined;
    const wasmUrl = baseUrl ? new URL('rust_counter_web_wasm.wasm', baseUrl) : 'rust_counter_web_wasm.wasm';
    WebAssembly.instantiateStreaming(fetch(wasmUrl), {})
      .then(result => {
        wasmExports = result.instance.exports;
      })
      .catch(() => {});
  }
}

function nextValue(value) {
  maybeLoadWasm();
  if (wasmExports && typeof wasmExports.rust_counter_increment === 'function') {
    return wasmExports.rust_counter_increment(value);
  }
  return value + 1;
}

function emit(value) {
  observers.slice().forEach(observer => observer(value));
}

function formatCountLabel(prefix, value) {
  return `${prefix}: ${Math.trunc(value)}`;
}

function createSubscription(observer) {
  let closed = false;
  return {
    get closed() {
      return closed;
    },
    unsubscribe() {
      if (closed) {
        return;
      }
      closed = true;
      const index = observers.indexOf(observer);
      if (index >= 0) {
        observers.splice(index, 1);
      }
    },
  };
}

function count() {
  return {
    subscribe(nextOrObserver) {
      const observer = typeof nextOrObserver === 'function' ? nextOrObserver : nextOrObserver && nextOrObserver.next;
      if (typeof observer !== 'function') {
        return createSubscription(() => {});
      }

      observers.push(observer);
      observer(countValue);
      return createSubscription(observer);
    },
  };
}

function increment() {
  countValue = nextValue(countValue);
  emit(countValue);
}

function describeAfterIncrement(formatter) {
  countValue = nextValue(countValue);
  emit(countValue);
  return formatter(countValue);
}

function formatCount(prefix, value) {
  return formatCountLabel(prefix, value);
}

function formatCountAsync(value) {
  return Promise.resolve(`Async Rust count: ${Math.trunc(value)}`);
}

function labelBytes(label) {
  if (typeof TextEncoder !== 'undefined') {
    return new TextEncoder().encode(label);
  }
  return Uint8Array.from(Array.prototype.map.call(label, char => char.charCodeAt(0) & 0xff));
}

function payloadSize(payload) {
  return payload.length;
}

function echoPayload(payload) {
  return {
    label: `${payload.label} echoed`,
    value: payload.value + 1,
    mode: payload.mode,
  };
}

function modeLabel(mode) {
  return `Rust enum mode: ${mode}`;
}

maybeLoadWasm();

module.exports = {
  count,
  increment,
  describeAfterIncrement,
  formatCount,
  formatCountAsync,
  labelBytes,
  payloadSize,
  echoPayload,
  modeLabel,
};
