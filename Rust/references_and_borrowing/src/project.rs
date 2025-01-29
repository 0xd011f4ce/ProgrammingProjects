fn add_flour (mut meal: String) -> String
{
		meal.push_str (" Add flour");
		meal
}

fn add_sugar (meal: &mut String)
{
		meal.push_str (" Add sugar");
}

fn show_my_meal (meal: &String)
{
		println! ("Meal steps: {meal}");
}

fn main() {
		let mut current_meal = String::new ();
		current_meal = add_flour (current_meal);
		add_sugar (&mut current_meal);
		show_my_meal (&current_meal); // this doesn't change ownership, as we are passing a reference, and not the variable itself

		// multiple immutable references
		let car = String::from ("Red");
		let ref1 = &car;
		let ref2 = &car;
		println! ("{ref1} and {ref2}");

		// Mutable reference restrictions
		let mut car = String::from ("Red");
		let ref1 = &mut car;
		// let ref2 = &car; this is not allowed
		println! ("{ref1}");

		let languages = [String::from ("Rust"), String::from ("Javascript")];
		let first = &languages [0]; // we need to borrow the value, we cannot change its ownership (also applies for tuples)
		println! ("{first} and {languages:?}");
}
