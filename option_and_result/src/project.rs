#[derive (Debug, Copy, Clone)]
enum MyOption
{
		Some (i32),
		None
}

// methods
impl MyOption
{
		fn unwrap (self) -> i32
		{
				match self
				{
						MyOption::Some (value) => value,
						MyOption::None => panic! ("Uh oh")
				}
		}

		fn unwrap_or (self, fallback_value: i32) -> i32
		{
				match self
				{
						MyOption::Some (value) => value,
						MyOption::None => fallback_value
				}
		}
}

// functions
fn play (instrument_option: Option<&String>)
{
		match instrument_option
		{
				Option::Some (instrument) => {
						println! ("Playing the {instrument}");
				},
				Option::None => {
						println! ("Singing with my voice");
				}
		}
}

fn is_item_in_stock (item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool>
{
		if item_is_in_system && item_is_in_stock
		{
				Option::Some (true)
		}
		else if item_is_in_system
		{
				Option::Some (false)
		}
		else
		{
				Option::None
		}
}

fn divide (numerator: f64, denominator: f64) -> Result<f64, String>
{
		if denominator == 0.0
		{
				Err (String::from ("Cannot divide by zero"))
		}
		else
		{
				Ok (numerator / denominator)
		}
}

fn operation (great_success: bool) -> Result<&'static str, &'static str>
{
		if great_success
		{
				Ok ("Success")
		}
		else
		{
				Err ("Error")
		}
}

fn main() {
		let a = Option::Some (5);
		let b = Option::Some ("hello");
		let d: Option<&str> = Option::None;

		let musical_instruments = [
				String::from ("Guitar"),
				String::from ("Drums"),
				String::from ("Bass")
		];
		let bass = musical_instruments.get (2); // this returns an option enum
		play (bass); // this doesn't transfer ownership

		// let valid_instrument = bass.unwrap ();

		let invalid_instrument = musical_instruments.get (100);
		println! ("{:?}", invalid_instrument);

		// invalid_instrument.unwrap (); -- this will lead to a panic
		// invalid_instrument.expect("Unable to retrieve musical instrument"); -- this will display a custom message, program will still panic

		// this is a common solution to the unwrap problem
		play (invalid_instrument);

		let availability = is_item_in_stock (true, true);
		match availability
		{
				Some (true) => println! ("Yes, the item is in stock"), // we can skip the `Option::' prefix
				Some (false) => println! ("Nope, the item is not in stock"),
				None => println! ("Your item doesn't exist in our system")
		}

		let present_value = Option::Some (13);
		let missing_value: Option<i32> = Option::None;

		println! ("{}", present_value.unwrap_or (0)); // unwrap or returns a fallback value
		println! ("{}", missing_value.unwrap_or (0));

		let some_option = MyOption::Some (100);
		println! ("{}", some_option.unwrap ());

		let none_option = MyOption::None;
		println! ("{}", none_option.unwrap_or (0));

		let ok: Result<i32, &str> = Result::Ok (5);
		let disaster: Result<i32, &str> = Result::Err ("Something went wrong");

		let text = "50";
		let text_as_number = text.parse::<i32> ();
		println! ("{:?}", text_as_number); // returns Ok

		let text = "Alabama";
		let text_as_number = text.parse::<i32> ();
		println! ("{:?}", text_as_number); // returns Err

		let result = divide (10.0, 2.0);
		println! ("{}", result.is_ok ());
		println! ("{}", result.is_err ());

		let my_result = operation (true);
		let content = match my_result
		{
				Ok (message) => message,
				Err (error) => error
		};
}
