#![deny(warnings)]

pub mod ecdsa_keys;

#[cfg(feature = "with_precompile_utils")]
pub use precompile_utils;

#[cfg(feature = "with_common_precompiles")]
pub use pub_use::*;

#[cfg(feature = "with_common_precompiles")]
mod pub_use {
    pub use ruc_evm_precompile_blake2 as evm_precompile_blake2;
    pub use ruc_evm_precompile_bn128 as evm_precompile_bn128;
    pub use ruc_evm_precompile_curve25519 as evm_precompile_curve25519;
    pub use ruc_evm_precompile_ed25519 as evm_precompile_ed25519;
    pub use ruc_evm_precompile_modexp as evm_precompile_modexp;
    pub use ruc_evm_precompile_sha3fips as evm_precompile_sha3fips;
    pub use ruc_evm_precompile_simple as evm_precompile_simple;
    pub use ruc_evm_test_vector_support as evm_test_vector_support;
    pub use ruc_fp_evm as fp_evm;
}
