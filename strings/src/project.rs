use std::io;

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

		let first_name = String::from ("Joseph");

		let icon = format! ("{first_name} {last_name}");
		println! ("{}", icon);

		let mut music_genre = "			Rock, Metal, Country, Rap			";
		println! ("{}", music_genre.trim ());
		println! ("{}", music_genre.trim_start ());
		println! ("{}", music_genre.trim_end ());

		music_genre = music_genre.trim ();
		println! ("{}", music_genre.to_uppercase ());
		println! ("{}", music_genre.to_lowercase ());
		println! ("{}", music_genre.replace ("a", "@"));

		let genres: Vec<&str> = music_genre.split (", ").collect ();
		println! ("{:?}", genres);

		let mut name = String::new ();
		println! ("What is your name? ");
		io::stdin ().read_line (&mut name).expect ("Failed to collect input from the user");
		println! ("Hello, {}", name.trim ());
}
