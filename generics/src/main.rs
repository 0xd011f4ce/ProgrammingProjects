#[derive (Debug)]
enum DigitalContent
{
		AudioFile,
		VideoFile
}

#[derive (Debug)]
struct ChatMessage<T>
{
		content: T,
		time: String
}

impl ChatMessage<DigitalContent>
{
		fn consume_entertainment (&self)
		{
				println! ("Watching the {:?}", self.content);
		}
}

impl<T> ChatMessage<T>
{
		fn retrieve_time (&self) -> String
		{
				self.time.clone ()
		}
}

fn main ()
{
		let message1 = ChatMessage {
				content: "Hello",
				time: String::from ("17:11")
		};

		let message2 = ChatMessage
		{
				content: String::from ("Howdy! How is it going?"),
				time: String::from ("17:11")
		};

		let message3 = ChatMessage
		{
				content: DigitalContent::VideoFile,
				time: String::from ("17:12")
		};

		message3.consume_entertainment ();

		println! ("{}", message1.retrieve_time ());
		println! ("{}", message2.retrieve_time ());
		println! ("{}", message3.retrieve_time ());
}
