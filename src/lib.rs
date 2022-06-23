mod util;

use num_bigint_dig::RandPrime;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[allow(non_snake_case)]
pub fn probablyPrime(size: usize) -> String {
    let mut random = rand::thread_rng();
    let prime = random.gen_prime(size);

    prime.to_str_radix(10)
}
