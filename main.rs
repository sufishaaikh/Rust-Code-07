#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}
#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}
impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content);
    }
}
impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}
fn main() {
    let message = ChatMessage {
        content: "Yo, Wassup",
        time: String::from("00:58 | 30-08-2025"),
    };
    println!("{}", message.retrieve_time());

    let notification = ChatMessage {
        content: String::from("Hi, You There?"),
        time: String::from("12:43 | 31-08-2025"),
    };
    println!("{}", notification.retrieve_time());

    let video = ChatMessage {
        content: DigitalContent::VideoFile,
        time: String::from("15:34 | 01-09-2025"),
    };
    println!("{}", video.retrieve_time());
    video.consume_entertainment();
}
