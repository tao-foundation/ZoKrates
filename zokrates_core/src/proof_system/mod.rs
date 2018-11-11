mod pghr13;
mod utils;

pub use self::pghr13::PGHR13;

use flat_absy::flat_variable::FlatVariable;
use field::Field;

pub trait ProofSystem {
	fn setup<T: Field>(
	    variables: Vec<FlatVariable>,
	    a: Vec<Vec<(usize, T)>>,
	    b: Vec<Vec<(usize, T)>>,
	    c: Vec<Vec<(usize, T)>>,
	    num_inputs: usize,
	    pk_path: &str,
	    vk_path: &str,
	) -> bool;

	fn generate_proof<T: Field>(
	    pk_path: &str,
	    proof_path: &str,
	    public_inputs: Vec<T>,
	    private_inputs: Vec<T>,
	) -> bool;
}