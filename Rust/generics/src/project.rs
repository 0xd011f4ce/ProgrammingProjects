fn identity<T> (value: T) -> T
{
		value
}

fn make_tuple<T, U> (first: T, second: U) -> (T, U)
{
		(first, second)
}

#[derive (Debug)]
struct TreasureChest<T>
{
		captain: String,
		treasure: T
}

enum Cheesesteak<T>
{
		Plain,
		Topping (T)
}

impl TreasureChest<String>
{
		fn clean_treasure (&mut self)
		{
				self.treasure = self.treasure.trim ().to_string ();
		}
}

impl TreasureChest<[&str; 3]>
{
		fn amount_of_treasure (&self) -> usize
		{
				self.treasure.len ()
		}
}

impl<T> TreasureChest<T>
{
		fn capital_captain (&self) -> String
		{
				self.captain.to_uppercase ()
		}
}

fn main() {
		println! ("{}", identity::<i32> (5));
		println! ("{}", identity::<f64> (3.14159));
		println! ("{}", identity::<&str> ("hello from the binary"));
		println! ("{}", identity::<String> (String::from ("hello from the heap")));

		println! ("{:?}", make_tuple ("hello", true));

		let mut gold_chest = TreasureChest
		{
				captain: String::from ("Firebeard"),
				treasure: String::from ("Gold")
		};
		gold_chest.clean_treasure ();
		println! ("{}", gold_chest.capital_captain ());

		let special_chest = TreasureChest
		{
				captain: String::from ("Bootyplunder"),
				treasure: [ "Gold", "Silver", "Platinum" ]
		};
		let treasures = special_chest.amount_of_treasure ();
		println! ("The special chest contains {treasures} treasures");
		println! ("{}", special_chest.capital_captain ());

		let mushroom = Cheesesteak::Topping ("mushroom");
		let onions = Cheesesteak::Topping ("onions".to_string ());
		let plain: Cheesesteak<String> = Cheesesteak::Plain;
}
