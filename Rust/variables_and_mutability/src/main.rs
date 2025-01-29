const TOUCHDOWN_POINTS:i32 = 6;

fn main ()
{
		let season:&str = "Fall";
		println! ("My favourite season is {0}", season);
		
		let mut points_scored:i32 = 28;
		println! ("The total points scored are: {points_scored}");

		points_scored = 35;
		println! ("The new total points scored are: {points_scored}");

		println! ("The touchdown points are {TOUCHDOWN_POINTS}");

		let event_time:&str = "06:00";
		println! ("The time of the event is {}", event_time);
		let event_time:i32 = 6;
		println! ("The time of the event as an integer is {event_time}");

		#[allow(unused_variables)]
		let favourite_beverage:&str = "Sparkling Water";
}
