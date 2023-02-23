pub mod air;
pub mod fri;

use air::polynomials::get_cp_and_tp;
use fri::fri_decommit::fri_decommit_layers;
use lambdaworks_crypto::fiat_shamir::transcript::Transcript;
use lambdaworks_math::polynomial::Polynomial;
use winterfell::{
    crypto::hashers::Blake3_256,
    math::{fields::f128::BaseElement, StarkField},
    prover::constraints::CompositionPoly,
    Air, AuxTraceRandElements, Serializable, Trace, TraceTable,
};

use lambdaworks_math::field::element::FieldElement;
use lambdaworks_math::{
    field::fields::u384_prime_field::{IsMontgomeryConfiguration, MontgomeryBackendPrimeField},
    unsigned_integer::element::U384,
};

type U384PrimeField = MontgomeryBackendPrimeField<crate::air::polynomials::MontgomeryConfig>;
type U384FieldElement = FieldElement<U384PrimeField>;
const MODULUS_MINUS_1: U384 =
    U384::from("800000000000011000000000000000000000000000000000000000000000000");

pub fn generate_vec_roots(subgroup_size: u64, coset_factor: u64) -> Vec<U384FieldElement> {
    let MODULUS_MINUS_1_FIELD: U384FieldElement = U384FieldElement::new(MODULUS_MINUS_1);
    let subgroup_size_u384: U384FieldElement = subgroup_size.into();
    let generator_field: U384FieldElement = 3.into();
    let coset_factor_u384: U384FieldElement = coset_factor.into();

    let exp = (MODULUS_MINUS_1_FIELD) / subgroup_size_u384;
    let exp_384 = *exp.value();

    let generator_of_subgroup = generator_field.pow(exp_384);

    let mut numbers = Vec::new();

    for exp in 0..subgroup_size {
        let ret = generator_of_subgroup.pow(exp) * &coset_factor_u384;
        numbers.push(ret.clone());
    }

    numbers
}

#[derive(Debug)]
pub struct StarkProof {
    // TODO: fill this when we know what a proof entails
}

pub use lambdaworks_crypto::merkle_tree::{DefaultHasher, MerkleTree};
pub type FriMerkleTree = MerkleTree<U384PrimeField, DefaultHasher>;

pub fn prove<A>(air: A, trace: TraceTable<A::BaseField>, pub_inputs: A::PublicInputs) -> StarkProof
where
    A: Air<BaseField = BaseElement>,
{
    let mut transcript = Transcript::new();
    // * Generate composition polynomials using Winterfell
    let (mut composition_poly, mut trace_poly) = get_cp_and_tp(air, trace, pub_inputs).unwrap();

    // * Generate Coset
    let roots_of_unity = crate::generate_vec_roots(1024, 1);

    // * Do Reed-Solomon on the trace and composition polynomials using some blowup factor
    let composition_poly_lde = composition_poly.evaluate_slice(roots_of_unity.as_slice());

    // * Commit to both polynomials using a Merkle Tree
    let commited_poly_lde = FriMerkleTree::build(composition_poly_lde.as_slice());

    // * Do FRI on the composition polynomials
    let lde_fri_commitment = crate::fri::fri(&mut composition_poly, &roots_of_unity);

    // * Sample q_1, ..., q_m using Fiat-Shamir
    // let q_1 = transcript.challenge();
    // TODO: Do this with Fiat Shamir
    let q_1: usize = rand::random();

    // * For every q_i, do FRI decommitment
    let decommitment = fri_decommit_layers(&lde_fri_commitment, q_1, &mut transcript);

    // * For every trace polynomial t_i, provide the evaluations on every q_i, q_i * g, q_i * g^2

    StarkProof {}
}

pub fn verify() {}

#[cfg(test)]
mod tests {
    use super::prove;
    use winterfell::{FieldExtension, ProofOptions};

    #[test]
    fn test_prove() {}
}
