pub mod common;
pub mod scs;
pub mod ssp;

mod prover;
mod serialization;
mod setup;
mod verifier;

pub use prover::{Proof, Prover};
pub use setup::{setup, ProvingKey, VerifyingKey};
pub use verifier::verify;
