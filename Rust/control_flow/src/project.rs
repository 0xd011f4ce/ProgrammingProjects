fn even_or_odd (number: i32)
{
		let result = if number % 2 == 0 { "even" } else { "odd" };
		println! ("The number is: {result}");
}

fn main ()
{
		// if and else-if
		let some_condition: bool = true;
		if some_condition
		{
				println! ("This line will be output!");
		}

		let season: &str = "Summer";

		if season == "Summer"
		{
				println! ("School's out!");
		}
		else if season == "Winter"
		{
				println! ("Brr, so cold!");
		}
		else
		{
				println! ("Lots of rain!");
		}

		even_or_odd (17);
		even_or_odd (100);

		let evaluation = true;

		let value = match evaluation
		{
				true => 20,
				false => 40
		};

		println! ("{value}");

		match season
		{
				"summer" => {
						println! ("School's out!");
				}

				"winter" => {
						println! ("Brr, so col!");
				}

				_ => {
						println! ("Lots of rain!");
				}
		}

		let number: i32 = 8;

		match number
		{
				2 | 4 | 6 | 8 => println! ("the number {number} is even!"),
				1 | 3 | 5 | 7 => println! ("the number {number} is odd"),
				_ => println! ("you've entered an invalid number")
		}

		match number {
				value if value % 2 == 0 => println! ("the number {value} is even!"),
				value if value % 2 != 0 => println! ("the number {value} is odd"),
				_ => unreachable! (),
		}

		let mut i = 21;
		while i > 0 {
				if i % 2 == 0
				{
						i -= 3;
						continue;
				}

				println! ("{i}");
				i -= 1;
		}
}

