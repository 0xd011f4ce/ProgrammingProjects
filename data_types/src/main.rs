#![allow (unused_variables)]

fn main()
{
		// Integers
		let eight_bit: i8 = 112;
		let sixteen_bit: i16 = -32500;
		let thirty_two_bit: i32 = 2147483646;
		let some_value = 20i8; // treat it as an i8
		let another_value: i32 = 1_428_720;
		let days: usize = 55;

		// Strings
		println! ("Dear Milena\nI am Kafka.");
		let raw_string = r"C:\My Documents\new\videos";
		println! ("{raw_string}");

		// Methods
		let value:i32 = -15;
		println! ("{}", value.abs ());
		println! ("{}", value.pow (2));

		let empty_space:&str = "								my content									";
		println! ("{}", empty_space.trim ());

		// Floats
		let pi:f64 = 3.14159;
		println! ("{}", pi.floor ());
		println! ("{}", pi.ceil ());
		println! ("{}", pi.round ());
		println! ("I am using format specifies: {:.2}", pi);

		// Casting
		let miles_away:i32 = 50;
		let miles_away_i8 = miles_away as i8;

		let miles_away = 100.329032;
		let miles_away_int = miles_away as i32;
		println! ("{miles_away_int}");

		// Math operations
		let addition = 5 + 4;
		let subtraction = 10 - 6;
		let multiplication = 3 * 4;
		let division = 5.0 / 3.0;
		let modulus = 7 % 2;

		println! ("\nAddition: {addition}\nSubtraction: {subtraction}\nMultiplication: {multiplication}\nDivision: {division}\nModulus: {modulus}");

		// Augmented assignment
		let mut year = 2024;
		year += 1;

		println! ("The new year is: {year}");

		// Booleans
		let is_handsome: bool = true;
		let is_silly: bool = false;

		println! ("{}", !true); // negation
		println! ("{}", 2 == 2); // equality
		println! ("{}", 1 != 2); // inequality
		println! ("{}", 13 == 13.0 as i32); // casting

		let purchased_ticket = true;
		let plane_on_time = true;
		let making_event = purchased_ticket && plane_on_time;
		println! ("It is {} that I will arrive as expected", making_event);

		let user_has_paid_for_subscription = false;
		let user_is_admin = true;
		let user_can_see_premium_experience = user_has_paid_for_subscription || user_is_admin;
		println! ("Can this user see my site? {user_can_see_premium_experience}");

		// Characters
		let first_initial: char = 'S';
}
