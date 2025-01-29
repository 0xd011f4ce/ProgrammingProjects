use fake::Dummy;

#[derive (Debug, Dummy)]
pub enum ProductCategory
{
		Ladder,
		Hammer
}

#[derive (Debug, Dummy)]
pub struct Item
{
		pub name: String,
		pub category: ProductCategory,
		pub quantity: u32
}		

impl Item
{
		pub fn new (name: String, category: ProductCategory, quantity: u32) -> Self
		{
				super::talk_to_manager (); // `super' represents the parent module
				
				Self
				{
						name,
						category,
						quantity
				}
		}
}
