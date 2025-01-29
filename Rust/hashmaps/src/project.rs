use std::collections::{HashMap, HashSet};

fn main ()
{
		let mut menu: HashMap<String, f64> = HashMap::new ();

		menu.insert (String::from ("Steak"), 29.99);
		menu.insert (String::from ("Tuna"), 29.99);
		menu.insert (String::from ("Burger"), 14.99);

		println! ("{:#?}", menu);

		let mut country_capitals = HashMap::<&str, &str>::new ();

		country_capitals.insert ("France", "Paris");
		country_capitals.insert ("Germany", "Berlin");

		println! ("{:#?}", country_capitals);

		let data = [
				("Bobby", 7),
				("Grant", 4),
				("Ben", 6)
		];

		let mut years_at_company = HashMap::from (data);
		println! ("{:#?}", years_at_company);

		let ben = years_at_company.remove ("Ben");
		println! ("{:?}", ben.unwrap ());
		println! ("{:#?}", years_at_company);

		let mut coffee_pairings: HashMap<&str, &str> = HashMap::new ();
		let drink = String::from ("Latte");
		let milk = String::from ("Oat Milk");
		coffee_pairings.insert (&drink, &milk); // we store a reference so we don't lose ownersip of these variables
		coffee_pairings.insert ("Flat White", "Almond Milk");
		println! ("{:?}", coffee_pairings);

		let value = coffee_pairings["Flat White"];
		println! ("{}", value);

		let value = coffee_pairings.get ("Cappuccino").copied ().unwrap_or ("Unknown Milk");
		println! ("{}", value);

		coffee_pairings.insert ("Latte", "Pistachio Milk"); // Latte will be replaced
		coffee_pairings.entry ("Cappuccino").or_insert ("Pistachio Milk"); // insert if it doesn't exist
		println! ("{:?}", coffee_pairings);

		let mut concert_queue: HashSet<&str> = HashSet::new ();
		concert_queue.insert ("Molly");
		concert_queue.insert ("Megan");
		println! ("{:?}", concert_queue);

		concert_queue.insert ("Molly"); // nothing will happen because it already exists
		println! ("{:?}", concert_queue);

		println! ("{}", concert_queue.contains ("Molly"));
		println! ("{}", concert_queue.contains ("Fred"));

		let mut movie_queue: HashSet<&str> = HashSet::new ();
		let mut concert_queue: HashSet<&str> = HashSet::new ();

		concert_queue.insert ("Boris");
		concert_queue.insert ("Melissa");

		movie_queue.insert ("Boris");
		movie_queue.insert ("Phil");

		println! ("{:?}", concert_queue.union (&movie_queue));
		println! ("{:?}", concert_queue.difference (&movie_queue));
		println! ("{:?}", movie_queue.difference (&concert_queue));
}
