//! # Name Generator
//! 
//! Crate for generating given name/family name pairs at random from weighted lists.
//! 
//! This crate is intended to be used only for my own game development projects, but I want to keep
//! it as generic and engine agnostic as possible in case someone else has a need for it, and in
//! case I ever need to change game engines.

use rand::Rng;
use rand_distr::{ Distribution, WeightedAliasIndex, weighted_alias::AliasableWeight };



/// Weighted list of singular names. Use if you don't need full names for your game (e.g. only a
/// character's family name is used.)
pub struct WeightedNameList<S, W>
where W: AliasableWeight
{
	names: Vec<S>,
	weights: WeightedAliasIndex<W>,
}

impl<S, W> WeightedNameList<S, W>
where W: AliasableWeight
{
	pub fn new(names: Vec<S>, weights: Vec<W>) -> Self
	{
		Self {
			names,
			weights: WeightedAliasIndex::new(weights).unwrap()
		}
	}

	/// Samples a single random entry from the list.
	pub fn sample<R>(&self, rng: &mut R) -> &S
	where R: Rng + ?Sized
	{
		&self.names[self.weights.sample(rng)]
	}
}

impl<R, S, W> From<Vec<(R, W)>> for WeightedNameList<S, W>
where R: Into<S>, S: Clone, W: AliasableWeight
{
	fn from(value: Vec<(R, W)>) -> Self {
		let mut name_vec = vec![];
		let mut weight_vec = vec![];
		for (name, weight) in value
		{
			name_vec.push(name.into());
			weight_vec.push(weight);
		}
		Self::new(name_vec, weight_vec)
	}
}

impl<R, S, W> From<(Vec<R>, Vec<W>)> for WeightedNameList<S, W>
where R: Into<S>, S: Clone, W: AliasableWeight
{
	fn from(value: (Vec<R>, Vec<W>)) -> Self {
		// separate input values
		let (input_name_vec, input_weight_vec) = value;
		// verify lengths match - panic otherwise
		if input_name_vec.len() != input_weight_vec.len()
		{
			panic!(
				"Vectors must be equal length. Name vec had length {}, while weight vec had length {}",
				input_name_vec.len(), input_weight_vec.len()
			);
		}
		// call `.into()` on names to convert them to type `S`
		let mut name_vec = Vec::new();
		for name in input_name_vec
		{
			name_vec.push(name.into());
		}
		// create and return struct
		Self::new(name_vec, input_weight_vec)
	}
}



#[cfg(test)]
mod tests
{
	use super::*;
	use approx::assert_ulps_eq;
	use rand::{thread_rng};

	const NAME_COUNT: usize = 3000;
	const EPSILON: f32 = 0.2;


	mod single_name
	{
		use super::*;

		#[test]
		/// Verify that randomly sampling enough names will result in roughly the same distribution defined by the weights
		/// 
		/// In this case, tests that the name `"Foo"` occurs approximately twice as often as `"Bar"`
		fn sample()
		{
			let mut rng = thread_rng();
			let test_data = vec![("Foo", 2), ("Bar", 1)];
			let name_list = WeightedNameList::<String, usize>::from(test_data);
			let mut count_foo = 0;
			let mut count_bar = 0;
			for _ in 0..NAME_COUNT
			{
				let name = name_list.sample(&mut rng);
				if name == &String::from("Foo") { count_foo += 1; }
				else if name == &String::from("Bar") { count_bar += 1; }
				else { panic!("Expected name to be either \"Foo\" or \"Bar\", but \"{}\" was returned", name); }
			}
			assert_ulps_eq!(2.0, count_foo as f32 / count_bar as f32, epsilon=EPSILON);
		}

		/// Verify that you can turn a vector of name/weight pairs into a [`WeightedNameList`]
		#[test]
		fn from_vec()
		{
			let test_data = vec![("Foo", 2), ("Bar", 3), ("Baz", 4)];
			let _result: WeightedNameList<String, usize> = WeightedNameList::from(test_data);
		}

		/// Verify that you can turn a vector of names and a separate vector of weights into a [`WeightedNameList`]
		#[test]
		fn from_vec_pair()
		{
			let test_data_names = vec!["Foo", "Bar", "Baz"];
			let test_data_weights = vec![1, 2, 3];
			let _result: WeightedNameList<String, usize> = WeightedNameList::from((test_data_names, test_data_weights));
		}

		/// Verify that when you try to turn two vectors of differing lengths into a [`WeightedNameList`], the program panics
		#[test]
		#[should_panic]
		fn from_vec_pair_length_mismatch()
		{
			let test_data_names = vec!["Foo", "Bar", "Baz"];
			let test_data_weights = vec![1, 2, 3, 4];
			let _result: WeightedNameList<String, usize> = WeightedNameList::from((test_data_names, test_data_weights));
		}
	}
}
