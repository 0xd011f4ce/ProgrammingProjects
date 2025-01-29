fn eat_meal (mut meal: String) -> String
{
		meal.clear ();
		meal
}

fn main ()
{
		let is_concert: bool = true;
		let is_event: bool = is_concert; // doesn't transfer ownership
		println! ("{} - {}", is_concert, is_event);

		let sushi: &str = "Salmon";
		let dinner: &str = sushi; // doesn't transfer ownership
		println! ("{} - {}", sushi, dinner);

		let sushi: String = String::from ("Salmon");
		let mut dinner: String = sushi; // this transfers ownership
		println! ("{dinner}");

		dinner = eat_meal (dinner); // we lost ownership of `dinner', that's why it has to be returned
		println! ("{}", dinner);
}
