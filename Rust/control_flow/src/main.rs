fn color_to_number (color: &str) -> i32
{
		let value = match color
		{
				"red" => 1,
				"green" => 2,
				"blue" => 3,
				_ => 0
		};

		value
}

fn factorial (num: i32) -> i32
{
		if num == 1
		{
				return 1;
		}

		num * factorial (num - 1)
}

fn factorial2 (num: i32) -> i32
{
		let mut result = 1;
		let mut current = num;

		while current > 0
		{
				result *= current;
				current -= 1;
		}

		result
}

fn main ()
{
		let color = color_to_number ("red");
		println! ("The id of the color is: {color}");
		
		let factorial_of_5 = factorial (5);
		println! ("{factorial_of_5}");
		println! ("{}", factorial2 (5));
}
