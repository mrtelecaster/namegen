//! # Name Generator
//! 
//! Crate for generating given name/family name pairs at random from weighted lists.
//! 
//! This crate is intended to be used only for my own game development projects, but I want to keep
//! it as generic and engine agnostic as possible in case someone else has a need for it, and in
//! case I ever need to change game engines.

use rand::Rng;
use rand_distr::{WeightedAliasIndex, weighted_alias::AliasableWeight, Distribution};



/// Weighted list of singular names. Use if you don't need full names for your game (e.g. only a
/// character's family name is used.)
pub struct WeightedNameList<S, W>
where W: AliasableWeight
{
	names: Vec<S>,
	weights: Vec<W>,
}

impl<S, W> WeightedNameList<S, W>
where W: AliasableWeight
{
	/// Inserts a name/weight pair into an existing
	pub fn insert(&mut self, name: S, weight: W)
	{
		self.names.push(name);
		self.weights.push(weight);
	}

	/// Adds a name to the list, with the given weight
	pub fn with_entry(mut self, name: S, weight: W) -> Self
	{
		self.insert(name, weight);
		self
	}

	/// Samples a single random entry from the list.
	/// 
	/// If more than 1 name needs to be generated at a time, [`sample_batch`](WeightedNameList::sample_batch)
	/// should be used over this function as it will run significantly faster than calling `sample`
	/// multiple times in a row.
	pub fn sample<R>(&self, rng: &mut R) -> &S
	where R: Rng + ?Sized
	{
		let aliased_weights = WeightedAliasIndex::new(self.weights.clone()).unwrap();
		&self.names[aliased_weights.sample(rng)]
	}

	/// Samples multiple names from the list in one call, returning them in a vector.
	/// 
	/// If more than 1 name needs to be generated at a time, this function should be used over
	/// [`sample`](WeightedNameList::sample) as it will run significantly faster.
	pub fn sample_batch<R>(&self, rng: &mut R, count: usize) -> Vec<&S>
	where R: Rng + ?Sized
	{
		let weights_index = WeightedAliasIndex::new(self.weights.clone()).unwrap();
		let mut names = Vec::new();
		for _ in 0..count
		{
			let index = weights_index.sample(rng);
			let name = &self.names[index];
			names.push(name);
		}
		names
	}
}

impl<S, W> Default for WeightedNameList<S, W>
where W: AliasableWeight
{
	fn default() -> Self
	{
		Self { names: Vec::default(), weights: Vec::default() }
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
	fn test_sample()
	{
		let mut rng = thread_rng();
		let name_list = WeightedNameList::<String, f32>::default()
			.with_entry(String::from("Foo"), 2.0)
			.with_entry(String::from("Bar"), 1.0);
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

	#[test]
	fn test_sample_batch()
	{
		let mut rng = thread_rng();
		let name_list = WeightedNameList::<String, f32>::default()
			.with_entry(String::from("Foo"), 2.0)
			.with_entry(String::from("Bar"), 1.0);
		let mut count_foo = 0;
		let mut count_bar = 0;
		let names = name_list.sample_batch(&mut rng, NAME_COUNT);
		for name in names
		{
			if name == &String::from("Foo") { count_foo += 1; }
			else if name == &String::from("Bar") { count_bar += 1; }
			else { panic!("Expected name to be either \"Foo\" or \"Bar\", but \"{}\" was returned", name); }
		}
		assert_ulps_eq!(2.0, count_foo as f32 / count_bar as f32, epsilon=EPSILON);
	}
}
