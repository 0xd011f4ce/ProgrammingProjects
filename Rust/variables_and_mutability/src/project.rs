// compiler directives
#![allow (unused_variables)]

// constants
const TAX_RATE: f64 = 7.25;

// type aliases
type Meters = i32;

fn main() {
		// variables
		let apples = 50;
		let oranges = 14 + 6;
		let _fruits = apples + oranges; // we add the _ to not get a warning when compiling

		println! ("My garden has {} apples and {} oranges.", apples, oranges);

		// mutable variables
		let mut gym_reps = 10;
		println! ("I plan to do {gym_reps} reps.");

		gym_reps = 15;
		println! ("Now, I plan to do {gym_reps} reps.");

		// variable shadowing
		let _grams_of_protein = "100.345";
		let _grams_of_protein = 100.345; // we are redeclaring the variable
		let _grams_of_protein = 100;

		println! ("I will take {_grams_of_protein} grams of protein.");

		println! ("The tax rate is {TAX_RATE}.");

		let mile_race_length: Meters = 1600;
		let two_mile_race_length: Meters = mile_race_length * 2;
		println! ("A one mile race length is {mile_race_length} meters long, and a two mile race length is {two_mile_race_length} meters long.");

		// compiler directives
		let unused_variable = 120;
}
