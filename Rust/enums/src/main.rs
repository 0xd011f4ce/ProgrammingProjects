#[derive (Debug)]
enum Tier
{
		Gold,
		Silver,
		Platinum
}

#[derive (Debug)]
enum Subscription
{
		Free,
		Basic (f64, u32), // (price_per_month, number_of_months)
		Premium { tier: Tier }
}

impl Subscription
{
		fn summarize (&self)
		{
				if let Subscription::Free = self
				{
						println! ("You have limited access to the site.");
				}
				else if let Subscription::Basic (price, months) = self
				{
						println! ("You have limited access to the site's premium features for {price} for {months} months");
				}
				else if let Subscription::Premium { tier } = self
				{
						println! ("Your tier is {tier:?}");
				}
		}
}

fn main ()
{
		let subscription1 = Subscription::Free;
		let subscription2 = Subscription::Basic (4.99, 12);
		let subscription3 = Subscription::Premium { tier: Tier::Gold };

		subscription1.summarize ();
		subscription2.summarize ();
		subscription3.summarize ();
}
