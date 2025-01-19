// enums
#[derive (Debug)]
enum CardSuit
{
		Hearts,
		Diamonds,
		Spades,
		Clubs
}

#[derive (Debug)]
enum PaymentMethodType
{
		CreditCard (String),
		DebitCard (String),
		PayPal { username: String, password: String } // struct variants
}

#[derive (Debug)]
enum Beans
{
		Pinto,
		Black
}

#[derive (Debug)]
enum Meat
{
		Chiken,
		Steak
}

#[derive (Debug)]
enum RestaurantItem
{
		Burrito { meat: Meat, beans: Beans },
		Bowl { meat: Meat, beans: Beans },
		VeganPlate
}

enum OperatingSystem
{
		Linux,
		MacOS,
		Windows
}

enum LaundryCycle
{
		Cold,
		Hot { temperature: u32 },
		Delicate (String)
}

#[derive (Debug)]
enum OnlineOrderStatus
{
		Ordered,
		Packed,
		Shipped,
		Delivered
}

enum Milk
{
		Lowfat (i32),
		Whole,
		NonDairy { kind: String }
}

// implementations
impl LaundryCycle
{
		fn wash_laundry (&self)
		{
				match self
				{
						LaundryCycle::Cold => println! ("Running the laundry with cold temperature"),
						LaundryCycle::Hot { temperature } => println! ("Running the laundry with a temperature of {temperature}"),
						LaundryCycle::Delicate (fabric_type) => println! ("Running the laundry with a delicate cycle for {fabric_type}")
				}
		}
}

impl OnlineOrderStatus
{
		fn check (&self)
		{
				match self
				{
						OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
								println! ("Your item is being prepped for shipment.");
						},
						OnlineOrderStatus::Delivered => {
								println! ("Your item has been delivered.");
						},
						other_status => {
								println! ("Your item is {other_status:?}");
						}
				}
		}
}

// functions
fn years_since_release (os: OperatingSystem) -> u32
{
		match os
		{
				OperatingSystem::Windows => 39,
				OperatingSystem::MacOS => 23,
				OperatingSystem::Linux => 34
		}
}

fn main() {
		let first_card = CardSuit::Hearts;
		let second_card = CardSuit::Spades;

		let visa = PaymentMethodType::CreditCard (String::from ("0034-5678-1234-0091"));
		let mastercard = PaymentMethodType::DebitCard (String::from ("1234-5678-90123-4567"));
		let paypal = PaymentMethodType::PayPal {
				username: String::from ("john.doe@example.com"),
				password: String::from ("password")
		};
		
		println! ("{:?}", visa);
		println! ("{:?}", mastercard);
		println! ("{:?}", paypal);

		let lunch = RestaurantItem::Burrito { meat: Meat::Steak, beans: Beans::Pinto };
		let dinner = RestaurantItem::Bowl { meat: Meat::Chiken, beans: Beans::Black };
		let abandoned_meal = RestaurantItem::VeganPlate;
		println! ("Lunch was {lunch:?} and dinner was {dinner:?}");
		println! ("Nobody ate {abandoned_meal:?}");

		let my_computer = OperatingSystem::Linux;
		let age = years_since_release (my_computer); // ownership is transferred
		println! ("My computer's operating system is {} years old", age);

		LaundryCycle::Cold.wash_laundry ();
		LaundryCycle::Hot { temperature: 100 }.wash_laundry ();
		LaundryCycle::Delicate ( String::from ("Silk") ).wash_laundry ();

		OnlineOrderStatus::Ordered.check ();
		OnlineOrderStatus::Delivered.check ();
		OnlineOrderStatus::Shipped.check ();

		let my_beverage = Milk::Whole;

		if let Milk::Whole = my_beverage {
				println! ("You have whole milk!");
		}

		let Milk::Lowfat (percent) = my_beverage else
		{
				println! ("You don't have a Lowfat milk");
				return; // we need to finish the execution of the function
		};
}
