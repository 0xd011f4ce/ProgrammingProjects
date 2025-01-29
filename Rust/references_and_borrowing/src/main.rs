fn start_trip () -> String
{
		String::from ("The plan is...")
}

fn visit_piladelphia (trip: &mut String)
{
		trip.push_str ("Philadelphia");
}

fn visit_new_york (trip: &mut String)
{
		trip.push_str (" New York");
}

fn visit_boston (trip: &mut String)
{
		trip.push_str (" Boston");
}

fn show_itinerary (trip: &String)
{
		println! ("{}", trip);
}

fn main ()
{
		let mut trip: String = start_trip ();
		visit_piladelphia (&mut trip);
		visit_new_york (&mut trip);
		visit_boston (&mut trip);
		trip.push ('.');
		show_itinerary (&trip);
}
