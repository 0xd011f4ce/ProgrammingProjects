fn main() {
		let mut pizza_diameters = Vec::<i32>::new ();
		pizza_diameters.push (16);
		pizza_diameters.push (18);
		pizza_diameters.insert (1, 4);
		println! ("{:?}", pizza_diameters);

		let last_diameter = pizza_diameters.pop ();
		println! ("{}", last_diameter.unwrap_or (0));

		pizza_diameters.remove (1);
		println! ("{:?}", pizza_diameters);

		let pepperoni = String::from ("Pepperoni");
		let mushroom = String::from ("Mushroom");
		let sausage = String::from ("Sausage");
		let mut pizza_toppings = vec![pepperoni, mushroom, sausage]; // ownership is moved

		let value = pizza_diameters [0]; // create a copy of the value
		println! ("{}", value);

		let reference = &pizza_toppings [2]; // creates a reference
		println! ("{}", reference);
		
		let pastas = vec!["Rigatoni", "Angel hair", "Fettucine"];
		println! ("{pastas:?}");

		let toppings_slice = &pizza_toppings[..2];
		println! ("{:?}", toppings_slice);

		let option = pizza_toppings.get (100); // returns a reference
		match option
		{
				Some (topping) => println! ("The topping is {topping}"),
				None => println! ("No value at that index position")
		}

		let mut delicious_toppings = pizza_toppings; // transfers ownership
		let topping_reference = &delicious_toppings[1];
		delicious_toppings.push (String::from ("Olives"));
		// println! ("The topping is {topping_reference}"); this won't compile

		delicious_toppings [1] = String::from ("Olives"); // overwrite an item
		println! ("{:?}", delicious_toppings);

		let target_topping = &mut delicious_toppings [2];
		target_topping.push_str (" and Meatballs");
		println! ("{:?}", delicious_toppings);

		let mut seasons: Vec<&str> = Vec::with_capacity (4);
		println! ("Length: {}. Capacity: {}", seasons.len (), seasons.capacity ());

		seasons.push ("Summer");
		seasons.push ("Fall");
		seasons.push ("Winter");
		seasons.push ("Spring");
		println! ("Length: {}. Capacity: {}", seasons.len (), seasons.capacity ());

		seasons.push ("Summer");
		println! ("Length: {}. Capacity: {}", seasons.len (), seasons.capacity ());
}
