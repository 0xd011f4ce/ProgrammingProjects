fn main() {
		// &str
		let pirate: &str = "Bloodhook";

		// String
		let sailer: String = String::from (pirate);
		let bad_guy: String = pirate.to_string ();

		println! ("{pirate} and {sailer} and {bad_guy}");

		let first_initial = &sailer [0..1];
		println! ("{first_initial}");

		let mut full_name = String::from ("Sylvester");
		let last_name = String::from ("Stallone");
		full_name.push (' ');
		full_name.push_str (&last_name); // & converts a String to a &str
		println! ("{full_name}");

		let first_name = String::from ("Linda");
		let full_name = first_name + &last_name;
		println! ("{}", full_name); // ownership is moved
}
