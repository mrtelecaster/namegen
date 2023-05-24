//! Premade name lists for easy use in game


use crate::{WeightedNameList, WeightedFullNameList};


/// Preset name lists by locale.
/// 
/// These are based on census data collected by world governments and may not perfectly represent
/// the cultures in those nations. Don't @ me.
pub mod locale
{
	use super::*;

	/// Preset list of names weighted by their frequency in the nation of Japan.
	/// 
	/// This is included as an example of a name set that requires the generic terms "given" and
	/// "family" names to be used, as "first" and "last" names are reversed in japanese, making
	/// terms like these confusing or contradictory.
	pub mod jp
	{
		use super::*;

		/// Common given names in japan
		/// 
		/// Source: <https://forebears.io/japan/forenames>
		pub fn given_names() -> WeightedNameList<String, f32>
		{
			let names = vec![
				("Kenji", 1.545), ("Hiroshi", 1.511), ("Shigeru", 1.208), ("Sachiko", 1.042),
				("Masako", 1.009), ("Katsumi", 0.989), ("Yoko", 0.959), ("Michiko", 0.911),
				("Toshio", 0.871), ("Yoshiko", 0.871), ("Hiromi", 0.830), ("Hiroko", 0.826),
				("Yoshio", 0.790), ("Kazuo", 0.760), ("Akira", 0.753), ("Keiko", 0.739),
				("Hisako", 0.728), ("Yoshimi", 0.705), ("Fumiko", 0.675), ("Masao", 0.671),
			];
			WeightedNameList::from(names)
		}

		/// Common family names in Japan
		/// 
		/// Source: <https://forebears.io/japan/surnames>
		pub fn family_names() -> WeightedNameList<String, f32>
		{
			let names = vec![
				("Sato", 1.957), ("Suzuki", 1.889), ("Tanaka", 1.414), ("Watanabe", 1.364),
				("Takahashi", 1.343), ("Ito", 1.240), ("Yamamoto", 1.131), ("Nakamura", 1.124),
				("Kobayashi", 1.075), ("Saito", 1.038), ("Kato", 0.936), ("Yoshida", 0.867),
				("Yamada", 0.848), ("Sasaki", 0.707), ("Matsumoto", 0.685), ("Yamaguchi", 0.674),
				("Inoue", 0.649), ("Kimura", 0.601), ("Shimizu", 0.574), ("Hayashi", 0.572),
			];
			WeightedNameList::from(names)
		}

		/// Common names in Japan
		/// 
		/// Source: see [`given_names`] and [`family_names`]
		pub fn full_names() -> WeightedFullNameList<String, f32>
		{
			WeightedFullNameList::new(given_names(), family_names())
		}
	}

	/// Preset list of names weighted by their frequency in the Russian Federation
	pub mod ru
	{
		use super::*;

		/// Most common given names in Russia
		/// 
		/// Source: <https://forebears.io/russia/forenames>
		pub fn given_names() -> WeightedNameList<String, f32>
		{
			let names = vec![
				("Sergey", 4.943), ("Aleksandr", 4.530), ("Elena", 4.312), ("Tatyana", 3.744),
				("Olga", 3.609), ("Natalya", 3.605), ("Andrey", 3.487), ("Ekaterina", 3.285),
				("Dmitriy", 3.196), ("Irina", 3.030), ("Vladimir", 2.940), ("Aleksey", 2.850),
				("Svetlana", 2.768), ("Anastasiya", 2.769), ("Anna", 2.278), ("Maksim", 1.910),
				("Marina", 1.882), ("Ivan", 1.834), ("Evgeniy", 1.799), ("Alexander", 1.748),
			];
			WeightedNameList::from(names)
		}

		/// Most common family names in the Russian Federation
		/// 
		/// Source: <https://forebears.io/russia/surnames>
		pub fn family_names() -> WeightedNameList<String, f32>
		{
			let names = vec![
				("Ivanova", 0.928), ("Ivanov", 0.881), ("Kuznetsova", 0.454), ("Kuznetsov", 0.437),
				("Petrov", 0.430), ("Smirnova", 0.428), ("Magomedov", 0.385), ("Petrova", 0.383),
				("Smirnov", 0.366), ("Popov", 0.366), ("Popova", 0.366), ("Volkova", 0.304),
				("Novikova", 0.258), ("Morozova", 0.240), ("Sokolova", 0.230), ("Pavlova", 0.223),
				("Romanova", 0.222), ("Volkov", 0.219), ("Shevchenko", 0.218), ("Andreeva", 0.216),
			];
			WeightedNameList::from(names)
		}

		/// Most common names in the Russian Federation
		/// 
		/// Source: see [`given_names`] and [`full_names`]
		pub fn full_names() -> WeightedFullNameList<String, f32>
		{
			WeightedFullNameList::new(given_names(), family_names())
		}
	}

	/// Preset name lists weighted by their frequency in the United States of America
	pub mod us
	{
		use super::*;

		/// Premade list of given names in the United States of America
		/// 
		/// Source: <https://namecensus.com/first-names/>
		pub fn given_names() -> WeightedNameList<String, f32>
		{
			let names = vec![
				("James", 10.836), ("John", 10.682), ("Robert", 10.264), ("Mary", 8.586),
				("Michael", 8.586), ("William", 8.004), ("David", 7.717), ("Richard", 5.561),
				("Charles", 4.974), ("Joseph", 4.585), ("Thomas", 4.507), ("Patricia", 3.504),
				("Linda", 3.380), ("Barbara", 3.200), ("Elizabeth", 3.060), ("Jennifer", 3.044),
				("Maria", 2.704), ("Susan", 2.593), ("Margaret", 2.508), ("Dorothy", 2.374),
				 
			];
			WeightedNameList::from(names)
		}

		/// Premade list of family nams in the United States of America
		/// 
		/// Source: <https://www.thoughtco.com/most-common-us-surnames-1422656>
		pub fn family_names() -> WeightedNameList<String, f32>
		{
			let names = vec![
				("Smith", 2.443), ("Johnson", 1.933), ("Williams", 1.625), ("Brown", 1.437),
				("Jones", 1.425), ("Garcia", 1.166), ("Miller", 1.161), ("Davis", 1.116),
				("Rodriguez", 1.095), ("Martinez", 1.060), ("Hernandez", 1.040), ("Lopez", 0.875),
				("Gonzalez", 0.841), ("Wilson", 0.802), ("Anderson", 0.784), ("Thomas", 0.756),
				("Taylor", 0.751), ("Moore", 0.724), ("Jackson", 0.708), ("Martin", 0.703),
			];
			WeightedNameList::from(names)
		}

		/// Premade list of full names in the United States of America
		/// 
		/// Sources: see [`given_names`] and [`family_names`]
		pub fn full_names() -> WeightedFullNameList<String, f32>
		{
			WeightedFullNameList::new(given_names(), family_names())
		}
	}
}