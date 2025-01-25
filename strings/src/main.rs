use std::io;

fn make_money (string: &mut String) -> &mut String
{
		string.push_str ("$$$");
		string
}

fn trim_and_capitalize (string: &str) -> String
{
		let new_string: String = string.trim ().to_uppercase ();
		new_string
}

fn elements (string: &str) -> Vec<&str>
{
		let splitten: Vec<&str> = string.split ("!").collect ();
		splitten
}

fn get_identity () -> String
{
		let mut result = String::new ();
		let stdin = io::stdin ();

		println! ("Please enter your first name: ");
		let mut first_name = String::new ();
		stdin.read_line (&mut first_name).expect ("Failed to read your first name :c");

		println! ("Please enter your last name: ");
		let mut last_name = String::new ();
		stdin.read_line (&mut last_name).expect ("Failed to read your last name :c");

		result = format! ("{} {}", first_name.trim (), last_name.trim ());

		result
}

fn main ()
{
		let mut my_string = String::from ("were so gangsta");
		make_money (&mut my_string);

		let capitalize = trim_and_capitalize ("Capitalize thissss");

		let precious_rocks = "Gold!Silver!Platinum";
		let splitten_rocks = elements (precious_rocks);

		let name = get_identity ();

		println! ("{}", my_string);
		println! ("{}", capitalize);
		println! ("{:#?}", splitten_rocks);
		println! ("{}", name);
}
