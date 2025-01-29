use std::{ fmt, io::{ self, stdin, stdout } };
use std::collections::*; // import every name from this module to this file

use fake::{ Fake, Faker };

use warehouse::{ products, FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER };

/// Primary entry point into our warehouse program
fn main()
{
		println! ("Our managers are {} and {}. We have {} square feet of floor space", INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE);

		let fake_item: products::Item = Faker.fake ();
		println! ("{:?}", fake_item);

		let random_category: products::ProductCategory = Faker.fake ();
		println! ("{:?}", random_category);
}
