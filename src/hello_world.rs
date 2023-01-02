use crate::Runner;

pub struct HelloWorld {}

impl Runner for HelloWorld {
    fn run(&self) {
        println!("Hello world");
    }

    fn train(&self) {
        println!("Hello World!");
        println!("I'm a Rustacean");
    }
}
