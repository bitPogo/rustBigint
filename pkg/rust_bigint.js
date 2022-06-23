import * as wasm from "./rust_bigint_bg.wasm";

export function probablyPrime(size) {
    return wasm.probablyPrime(size);
}
