#![allow (unused_variables)]

fn main ()
{
		let leet_variable: i32 = 1_337;
		let another_variable = leet_variable as i16;
		let float_variable: f64 = 100.238940;

		println! ("{:.3}", float_variable);

		let with_milk: bool = true;
		let with_sugar: bool = true;
		let is_my_type_of_coffe: bool = with_milk && with_sugar;
		let is_acceptable_coffe: bool = with_milk || with_sugar;

		let my_arr: [i8; 4] = [ 1, 2, 3, 4 ];
		println! ("{:#?}", my_arr);

		let my_tuple = ( 10, 3.14159, true, my_arr );
		println! ("{:#?}", my_tuple);
}
