// use crate::{
//     constraints::{
//         boundary::{BoundaryConstraint, BoundaryConstraints},
//         transition::TransitionConstraint,
//     },
//     context::AirContext,
//     frame::Frame,
//     proof::options::ProofOptions,
//     trace::TraceTable,
//     traits::AIR,
// };
// use lambdaworks_math::field::{element::FieldElement, traits::IsFFTField};
// use std::marker::PhantomData;

// #[derive(Clone)]
// struct FibConstraint<F: IsFFTField> {
//     phantom: PhantomData<F>,
// }

// impl<F: IsFFTField> FibConstraint<F> {
//     pub fn new() -> Self {
//         Self {
//             phantom: PhantomData,
//         }
//     }
// }

// impl<F> TransitionConstraint<F> for FibConstraint<F>
// where
//     F: IsFFTField + Send + Sync,
// {
//     fn degree(&self) -> usize {
//         1
//     }

//     fn constraint_idx(&self) -> usize {
//         0
//     }

//     fn end_exemptions(&self) -> usize {
//         2
//     }

//     fn evaluate(
//         &self,
//         frame: &Frame<F>,
//         transition_evaluations: &mut [FieldElement<F>],
//         _periodic_values: &[FieldElement<F>],
//         _rap_challenges: &[FieldElement<F>],
//     ) {
//         let first_step = frame.get_evaluation_step(0);
//         let second_step = frame.get_evaluation_step(1);
//         let third_step = frame.get_evaluation_step(2);

//         let a0 = first_step.get_evaluation_element(0, 0);
//         let a1 = second_step.get_evaluation_element(0, 0);
//         let a2 = third_step.get_evaluation_element(0, 0);

//         let res = a2 - a1 - a0;

//         transition_evaluations[self.constraint_idx()] = res;
//     }
// }

// pub struct FibonacciAIR<F>
// where
//     F: IsFFTField,
// {
//     context: AirContext,
//     trace_length: usize,
//     pub_inputs: FibonacciPublicInputs<F>,
//     constraints: Vec<Box<dyn TransitionConstraint<F>>>,
// }

// #[derive(Clone, Debug)]
// pub struct FibonacciPublicInputs<F>
// where
//     F: IsFFTField,
// {
//     pub a0: FieldElement<F>,
//     pub a1: FieldElement<F>,
// }

// impl<F> AIR for FibonacciAIR<F>
// where
//     F: IsFFTField + Send + Sync + 'static,
// {
//     type Field = F;
// <<<<<<< HEAD
//     type PublicInputs = FibonacciPublicInputs<F>;
// =======
//     type FieldExtension = F;
//     type RAPChallenges = ();
//     type PublicInputs = FibonacciPublicInputs<Self::Field>;
// >>>>>>> main

//     const STEP_SIZE: usize = 1;

//     fn new(
//         trace_length: usize,
//         pub_inputs: &Self::PublicInputs,
//         proof_options: &ProofOptions,
//     ) -> Self {
//         let constraints: Vec<Box<dyn TransitionConstraint<F>>> =
//             vec![Box::new(FibConstraint::new())];

//         let context = AirContext {
//             proof_options: proof_options.clone(),
//             trace_columns: 1,
//             transition_exemptions: vec![2],
//             transition_offsets: vec![0, 1, 2],
//             num_transition_constraints: constraints.len(),
//         };

//         Self {
//             pub_inputs: pub_inputs.clone(),
//             context,
//             trace_length,
//             constraints,
//         }
//     }

//     fn composition_poly_degree_bound(&self) -> usize {
//         self.trace_length()
//     }

// <<<<<<< HEAD
//     fn transition_constraints(&self) -> &Vec<Box<dyn TransitionConstraint<F>>> {
//         &self.constraints
// =======
//     fn build_auxiliary_trace(
//         &self,
//         _main_trace: &TraceTable<Self::Field>,
//         _rap_challenges: &Self::RAPChallenges,
//     ) -> TraceTable<Self::Field> {
//         TraceTable::empty()
//     }

//     fn build_rap_challenges(
//         &self,
//         _transcript: &mut impl IsStarkTranscript<Self::FieldExtension>,
//     ) -> Self::RAPChallenges {
//     }

//     fn compute_transition_prover(
//         &self,
//         frame: &Frame<Self::Field, Self::FieldExtension>,
//         _periodic_values: &[FieldElement<Self::Field>],
//         _rap_challenges: &Self::RAPChallenges,
//     ) -> Vec<FieldElement<Self::Field>> {
//         let first_step = frame.get_evaluation_step(0);
//         let second_step = frame.get_evaluation_step(1);
//         let third_step = frame.get_evaluation_step(2);

//         let a0 = first_step.get_main_evaluation_element(0, 0);
//         let a1 = second_step.get_main_evaluation_element(0, 0);
//         let a2 = third_step.get_main_evaluation_element(0, 0);

//         vec![a2 - a1 - a0]
// >>>>>>> main
//     }

//     fn boundary_constraints(
//         &self,
//         _rap_challenges: &[FieldElement<Self::Field>],
//     ) -> BoundaryConstraints<Self::Field> {
//         let a0 = BoundaryConstraint::new_simple_main(0, self.pub_inputs.a0.clone());
//         let a1 = BoundaryConstraint::new_simple_main(1, self.pub_inputs.a1.clone());

//         BoundaryConstraints::from_constraints(vec![a0, a1])
//     }

//     fn context(&self) -> &AirContext {
//         &self.context
//     }

//     fn trace_length(&self) -> usize {
//         self.trace_length
//     }

//     fn pub_inputs(&self) -> &Self::PublicInputs {
//         &self.pub_inputs
//     }

//     fn compute_transition_verifier(
//         &self,
//         frame: &Frame<Self::FieldExtension, Self::FieldExtension>,
//         periodic_values: &[FieldElement<Self::FieldExtension>],
//         rap_challenges: &Self::RAPChallenges,
//     ) -> Vec<FieldElement<Self::Field>> {
//         self.compute_transition_prover(frame, periodic_values, rap_challenges)
//     }
// }

// pub fn fibonacci_trace<F: IsFFTField>(
//     initial_values: [FieldElement<F>; 2],
//     trace_length: usize,
// ) -> TraceTable<F> {
//     let mut ret: Vec<FieldElement<F>> = vec![];

//     ret.push(initial_values[0].clone());
//     ret.push(initial_values[1].clone());

//     for i in 2..(trace_length) {
//         ret.push(ret[i - 1].clone() + ret[i - 2].clone());
//     }

//     TraceTable::from_columns(vec![ret], 1)
// }
