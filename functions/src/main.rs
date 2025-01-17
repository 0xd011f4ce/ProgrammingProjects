fn apply_to_jobs (number: i32, title: &str)
{
		println! ("I am applying to {number} {title} jobs");
}

fn is_even (number: i32) -> bool
{
		number % 2 == 0
}

fn alphabets (string: &str) -> (bool, bool)
{
		let contains_a: bool = string.contains ("a");
		let contains_b: bool = string.contains ("b");
		(contains_a, contains_b)
}

fn main ()
{
		apply_to_jobs (35, "Rust Developer");

		let are_even: (bool, bool) = (is_even (8), is_even (9));
		println! ("{:?}", are_even);

		println! ("{:?}", alphabets ("aardvark"));
		println! ("{:?}", alphabets ("zoology"));
		println! ("{:?}", alphabets ("zebra"));
}
