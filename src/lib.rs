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



#[cfg(test)]
mod tests
{
	use super::*;
	use approx::assert_ulps_eq;
	use rand::{thread_rng};

	const NAME_COUNT: usize = 3000;
	const EPSILON: f32 = 0.2;

	#[test]
	fn test_single_sample()
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
}
