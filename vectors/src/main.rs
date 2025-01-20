#[derive (Debug)]
struct File
{
		name: String
}

#[derive (Debug)]
struct Folder
{
		name: String,
		contents: Vec<File>
}

impl Folder
{
		fn new (name: String) -> Folder
		{
				Folder {
						name,
						contents: Vec::new ()
				}
		}

		fn create_file (&mut self, name: String)
		{
				self.contents.push (File {
						name
				});
		}

		fn delete_file (&mut self, index: usize) -> File
		{
				self.contents.remove (index)
		}

		fn get_file (&self, index: usize) -> Option <&File>
		{
				self.contents.get (index)
		}
}

fn main ()
{
		let mut my_folder = Folder::new (String::from ("kewl_folder"));
		my_folder.create_file (String::from ("main.rs"));
		my_folder.create_file (String::from ("lib.rs"));
		println! ("{:?}", my_folder);

		my_folder.delete_file (1);
		println! ("{:?}", my_folder);

		let my_file = my_folder.get_file (0);
		match my_file
		{
				Some (file) => println! ("{:?}", file),
				None => println! ("There was no file")
		}
}
