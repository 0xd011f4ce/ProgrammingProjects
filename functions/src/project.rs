fn open_store (neighbourhood: &str)
{
		println! ("Opening my pizza store in the neighbourhood {neighbourhood}");
}

fn bake_pizza (number: i32, topping: &str)
{
		println! ("Baking {number} {topping} pizzas");
}

fn swim_in_profit ()
{
		println! ("So much $$$, so little time");
}

fn square (number: i32) -> i32
{
		return number * number;
}

fn implicit_square (number: i32) -> i32
{
		number * number							// when there's no semicolon and no return keyword, it'll implicitly be treated as a return
}

fn main ()
{
		// Basic functions
		open_store ("Lulz World");
		bake_pizza (5, "Pineapple");
		swim_in_profit ();

		// Explicit return
		let result: i32 = square (5);
		println! ("The square of 5 is {result}");

		let result: i32 = implicit_square (13);
		println! ("The square of 13 is {result}");
}
