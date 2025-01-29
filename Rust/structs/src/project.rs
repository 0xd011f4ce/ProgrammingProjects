#[derive(Debug)] // Derive a default debug trait
struct Coffee
{
		price: f64,
		name: String,
		is_hot: bool
}

#[derive(Debug)]
struct ArpaviejasSong
{
		title: String,
		release_year: u32,
		duration_secs: u32
}

impl ArpaviejasSong
{
		fn new (title: String, release_year: u32, duration_secs: u32) -> Self // if we don't include self, this is an associate function
		{
				Self { title, release_year, duration_secs }
		}
}

impl ArpaviejasSong {
		// Immutable struct value (self parameter takes ownership)
		fn display_song_info (self: Self)
		{
				println! ("Title: {}", self.title);
				println! ("Release year: {}", self.release_year);
				println! ("Years since release: {}", self.year_since_release ());
				println! ("Duration: {}", self.duration_secs);
		}

		// Mutable struct value (self parameter takes ownership, has permission to mutate)
		fn double_length (mut self)
		{
				self.duration_secs *= 2;
		}

		// Immutable reference tot he struct instance (no ownership moved)
		fn display_song_info_reference (&self)
		{
				println! ("Title: {}", self.title);
				println! ("Release year: {}", self.release_year);
				println! ("Years since release: {}", self.year_since_release ());
				println! ("Duration: {}", self.duration_secs);
		}

		fn double_length_reference (&mut self)
		{
				self.duration_secs *= 2;
		}

		fn is_longer_than (&self, other: &Self) -> bool
		{
				self.duration_secs > other.duration_secs
		}

		fn year_since_release (&self) -> u32
		{
				2025 - self.release_year
		}
}

fn make_coffee (name: String, price: f64, is_hot: bool) -> Coffee
{
		Coffee {
				name,
				price,
				is_hot,
		}
}

fn drink_coffee (coffee: &Coffee)
{
		println! ("Drinking my delicious {}", coffee.name);
}

#[derive (Debug)]
struct Computer
{
		cpu: String,
		memory: u32,
		hard_drive_capacity: u32
}

impl Computer
{
		// the builder pattern allows us to do something like computer.update_cpu (...).upgrade_memory (...) etc
		fn new (cpu: String, memory: u32, hard_drive_capacity: u32) -> Self
		{
				Self {
						cpu,
						memory,
						hard_drive_capacity
				}
		}

		fn upgrade_cpu (&mut self, new_cpu: String) -> &mut Self
		{
				self.cpu = new_cpu;
				self
		}

		fn upgrade_memory (&mut self, new_memory: u32) -> &mut Self
		{
				self.memory = new_memory;
				self
		}

		fn upgrade_hard_drive_capacity (&mut self, new_capacity: u32) -> &mut Self
		{
				self.hard_drive_capacity = new_capacity;
				self
		}
}

// tuple struct (Hours, minutes)
struct ShortDuration (u32, u32);
struct LongDuration (u32, u32); // (Years, months)

// unit-like struct
struct Empty;

fn main() {
		let beverage = make_coffee(String::from ("Latte"), 4.99, true);

		let caramel_macchiato = Coffee {
				name: String::from ("Caramel Macchiato"),
				..beverage // copies the values from `beverage' to this instance
		};
		drink_coffee (&caramel_macchiato);

		// println! ("My {} this morning cost {}. It is {} that it was hot", beverage.name, beverage.price, beverage.is_hot);
		println! ("{:?}", caramel_macchiato);

		let mut song = ArpaviejasSong
		{
				title: String::from ("Ladron de Almas"),
				release_year: 2004,
				duration_secs: 252
		};

		let mut song2 = ArpaviejasSong
		{
				title: String::from ("Otro tema d arpaviejas"),
				release_year: 2025,
				duration_secs: 700
		};

		// song.display_song_info ();
		// song.double_length();
		song.display_song_info_reference ();
		song.double_length_reference ();
		println! ("{}", song.is_longer_than (&song2));

		let song3 = ArpaviejasSong::new (String::from ("Otro tema d arpaviejas"), 2004, 192);

		let mut computer = Computer::new (String::from ("M3 Max"), 64, 512);
		computer.upgrade_cpu(String::from ("M4 Max")).upgrade_memory (128).upgrade_hard_drive_capacity(1024);
		println! ("{:?}", computer);

		let work_shift = ShortDuration (8, 0);
		println! ("{} hours {} minutes", work_shift.0, work_shift.1);

		let era = LongDuration (5, 3);
		println! ("{} years {} months", era.0, era.1);

		let my_empty_struct = Empty;
}
