fn do_hero_stuff (hero_name: &str)
{
		println! ("{hero_name} saves the day");
}

fn print_length (reference: &[i32])
{
		println! ("{}", reference.len ());
}

fn main() {
		let action_hero = String::from ("Arnold Schwarzenegger");
		let first_name = &action_hero [..6]; // 0..6
		let last_name = &action_hero [7..]; // 0..-end of the string
		println! ("{first_name} {last_name}");

		do_hero_stuff (&action_hero); // &str covers both &str and String

		let values = [4, 8, 15, 16, 23, 42];
		let my_slice = &values [0..3];
		let regular_reference = &values;
		print_length (my_slice);
		print_length (regular_reference);

		let mut my_array = [10, 15, 20, 25, 30];
		let my_slice = &mut my_array [2..4];
		println! ("My slice: {:?}", my_slice);

		my_slice [0] = 100;
		println! ("My slice: {:?}", my_slice);
}
