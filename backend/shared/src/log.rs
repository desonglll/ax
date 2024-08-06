use colored::Colorize;

pub struct Log {}

impl Log {
    pub fn system(message: String) -> () {
        println!("{}", message.blue())
    }

    pub fn operation(message: String) -> () {
        println!("{}", message.blue())
    }
    pub fn success(message: String) -> () {
        println!("{}", message.green())
    }
    pub fn warning(message: String) -> () {
        println!("{}", message.yellow())
    }
    pub fn error(message: String) -> () {
        println!("{}", message.red())
    }
}