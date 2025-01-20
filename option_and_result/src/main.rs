#[derive (Debug)]
struct Food
{
		name: String
}

#[derive (Debug)]
struct Restaurant
{
		reservations: u32,
		has_mice_infestation: bool
}

impl Restaurant
{
		fn chef_special (&self) -> Option<Food>
		{
				if self.has_mice_infestation
				{
						return None;
				}

				if self.reservations < 12
				{
						return Some (Food {
								name: String::from ("Uni Sashimi")
						});
				}
				else
				{
						return Some (Food {
								name: String::from ("Strip Steak")
						})
				}
		}

		fn deliver_burger (&self, address: &str) -> Result<Food, String>
		{
				if self.has_mice_infestation
				{
						return Err (String::from ("Sorry, we have a mice problem"));
				}
				else if address.is_empty ()
				{
						return Err (String::from ("No delivery address specified"));
				}
				else
				{
						return Ok (Food {
								name: String::from ("Burger")
						});
				}
		}
}

fn main ()
{
		let my_restaurant = Restaurant {
				reservations: 11,
				has_mice_infestation: true
		};
		println! ("{:?}", my_restaurant.chef_special ());
		println! ("{:?}", my_restaurant.deliver_burger ("123 Elm Street"));

		let another_restaurant = Restaurant {
				reservations: 15,
				has_mice_infestation: false
		};
		println! ("{:?}", another_restaurant.chef_special ());
		println! ("{:?}", another_restaurant.deliver_burger (""));
		println! ("{:?}", another_restaurant.deliver_burger ("123 Elm Street"));
}
