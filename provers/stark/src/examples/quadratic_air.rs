// use std::marker::PhantomData;

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

// #[derive(Clone)]
// struct QuadraticConstraint<F: IsFFTField> {
//     phantom: PhantomData<F>,
// }

// impl<F: IsFFTField> QuadraticConstraint<F> {
//     pub fn new() -> Self {
//         Self {
//             phantom: PhantomData,
//         }
//     }
// }

// impl<F> TransitionConstraint<F> for QuadraticConstraint<F>
// where
//     F: IsFFTField + Send + Sync,
// {
//     fn degree(&self) -> usize {
//         2
//     }

//     fn constraint_idx(&self) -> usize {
//         0
//     }

//     fn end_exemptions(&self) -> usize {
//         1
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

//         let x = first_step.get_evaluation_element(0, 0);
//         let x_squared = second_step.get_evaluation_element(0, 0);

//         let res = x_squared - x * x;

//         transition_evaluations[self.constraint_idx()] = res;
//     }
// }

// pub struct QuadraticAIR<F>
// where
//     F: IsFFTField,
// {
//     context: AirContext,
//     trace_length: usize,
//     pub_inputs: QuadraticPublicInputs<F>,
//     constraints: Vec<Box<dyn TransitionConstraint<F>>>,
// }

// #[derive(Clone, Debug)]
// pub struct QuadraticPublicInputs<F>
// where
//     F: IsFFTField,
// {
//     pub a0: FieldElement<F>,
// }

// impl<F> AIR for QuadraticAIR<F>
// where
//     F: IsFFTField + Send + Sync + 'static,
// {
//     type Field = F;
// <<<<<<< HEAD
// =======
//     type FieldExtension = F;
//     type RAPChallenges = ();
// >>>>>>> main
//     type PublicInputs = QuadraticPublicInputs<Self::Field>;

//     const STEP_SIZE: usize = 1;

//     fn new(
//         trace_length: usize,
//         pub_inputs: &Self::PublicInputs,
//         proof_options: &ProofOptions,
//     ) -> Self {
//         let constraints: Vec<Box<dyn TransitionConstraint<Self::Field>>> =
//             vec![Box::new(QuadraticConstraint::new())];

//         let context = AirContext {
//             proof_options: proof_options.clone(),
//             trace_columns: 1,
//             transition_exemptions: vec![1],
//             transition_offsets: vec![0, 1],
//             num_transition_constraints: constraints.len(),
//         };

//         Self {
//             trace_length,
//             context,
//             pub_inputs: pub_inputs.clone(),
//             constraints,
//         }
//     }

// <<<<<<< HEAD
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

//         let x = first_step.get_main_evaluation_element(0, 0);
//         let x_squared = second_step.get_main_evaluation_element(0, 0);

//         vec![x_squared - x * x]
//     }

//     fn number_auxiliary_rap_columns(&self) -> usize {
//         0
//     }

// >>>>>>> main
//     fn boundary_constraints(
//         &self,
//         _rap_challenges: &[FieldElement<Self::Field>],
//     ) -> BoundaryConstraints<Self::Field> {
//         let a0 = BoundaryConstraint::new_simple_main(0, self.pub_inputs.a0.clone());

//         BoundaryConstraints::from_constraints(vec![a0])
//     }

//     fn transition_constraints(&self) -> &Vec<Box<dyn TransitionConstraint<Self::Field>>> {
//         &self.constraints
//     }

//     fn context(&self) -> &AirContext {
//         &self.context
//     }

//     fn composition_poly_degree_bound(&self) -> usize {
//         2 * self.trace_length()
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

// pub fn quadratic_trace<F: IsFFTField>(
//     initial_value: FieldElement<F>,
//     trace_length: usize,
// ) -> TraceTable<F> {
//     let mut ret: Vec<FieldElement<F>> = vec![];

//     ret.push(initial_value);

//     for i in 1..(trace_length) {
//         ret.push(ret[i - 1].clone() * ret[i - 1].clone());
//     }

//     TraceTable::from_columns(vec![ret], 1)
// }
