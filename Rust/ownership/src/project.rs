fn print_my_value (value: String)
{
		println! ("Your value is {value}");
}

fn add_fries (mut meal: String)
{
		meal.push_str (" and Fries");
}

fn bake_cake () -> String{
		String::from ("Chocolate Mousee")
}

fn add_flour (mut meal: String)
{
		meal.push_str (" Add flour");
}

fn add_sugar (mut meal: String)
{
		meal.push_str (" Add Sugar");
}

fn main() {
		let mut name = String::from ("Boris"); // the String datatype is a string stored in the heap
		name.push_str (" Pask");

		println! ("{name}");

		let person = String::from ("Boris");
		let genius = person; // moves ownership from `person' to `genius'
		drop (genius); // free genius

		let person = String::from ("Boris");
		let genius = person.clone (); // copies the heap value, doesn't change its ownership

		// borrowing
		let my_stack_value = 2;
		let my_integer_reference = &my_stack_value;

		let my_heap_value = String::from ("Toyota");
		let my_heap_reference = &my_heap_value;
		println! ("{}", *my_heap_reference);

		let oranges = String::from ("Oranges");
		print_my_value (oranges); // ownership is transferred

		let burger = String::from ("Burger");
		add_fries (burger);

		let cake = bake_cake (); // this will be the owner of the returned value
		println! ("I now have a {cake} cake");

		let current_meal = String::new ();
		add_flour (current_meal);
		// add_sugar (current_meal); this is invalid
}
