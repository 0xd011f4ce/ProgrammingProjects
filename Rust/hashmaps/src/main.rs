use std::collections::HashMap;

fn main ()
{
		let mut sauces_to_meals: HashMap<&str, Vec<&str>> = HashMap::from ([
				("Ketchup", vec!["French Fries", "Burguers", "Hot Dogs"]),
				("Mayonnaise", vec!["Sandwiches", "Burguers", "Coleslaw"])
		]);

		sauces_to_meals.insert ("Mustard", vec!["Hot dog", "Burguers", "Pretzels"]);

		println! ("{:?}", sauces_to_meals.remove ("Mayonnaise").unwrap ());
		println! ("{:?}", sauces_to_meals.get ("Mustard").unwrap ());

		sauces_to_meals.entry ("Soy Sauce").or_insert (vec!["Sushi", "Dumplings"]);
		println! ("{:#?}", sauces_to_meals);
}
