#[derive (Debug)]
struct Flight
{
		origin: String,
		destination: String,
		price: f64,
		passengers: u32
}

impl Flight
{
		fn new (origin: String, destination: String, price: f64, passengers: u32) -> Self
		{
				Self {
						origin,
						destination,
						price,
						passengers
				}
		}

		fn change_destination (&mut self, new_destination: String) -> &mut Self
		{
				self.destination = new_destination;
				self
		}

		fn increase_price (&mut self) -> &mut Self
		{
				self.price *= 1.2;
				self
		}

		fn itinerary (&self) -> &Self
		{
				println! ("{} -> {}", self.origin, self.destination);
				self
		}
}

fn main ()
{
		let mut my_flight = Flight::new (String::from ("LAX"), String::from ("COL"), 220.0, 600);
		my_flight.change_destination (String::from ("CAL")).increase_price ().itinerary ();

		let my_other_flight = Flight {
				origin: String::from ("AUX"),
				destination: String::from ("COL"),
				..my_flight
		};

		println! ("{:#?}", my_flight);
		println! ("{:#?}", my_other_flight);
}
