#![feature(type_name_of_val)]

use crate::array_slice::ArraySlice;
use crate::debug::Debug;
use crate::display::Display;
use crate::format_print::FormatPrint;
use crate::hello_world::HelloWorld;
use crate::literal_operator::LiteralOperator;
use crate::tuple::Tuple;
use std::any::type_name_of_val;

pub mod array_slice;
pub mod debug;
pub mod display;
pub mod format_print;
pub mod hello_world;
pub mod literal_operator;
pub mod tuple;

trait Runner {
    fn get_name(&self) -> String {
        type_name_of_val(&self).to_string()
    }
    fn run(&self) {
        println!("nothing to run");
    }
    fn train(&self) {
        println!("no trainings");
    }
}

pub fn run_all() {
    let all: Vec<Box<dyn Runner>> = vec![
        Box::new(HelloWorld {}),
        Box::new(FormatPrint {}),
        Box::new(Debug {}),
        Box::new(Display {}),
        Box::new(LiteralOperator {}),
        Box::new(Tuple {}),
        Box::new(ArraySlice {}),
    ];

    for r in all {
        println!("**************************");
        println!("Start {}", r.get_name());
        println!("=== Start Run ===");
        r.run();
        println!("=== End Run ===");
        println!("=== Start Train ===");
        r.train();
        println!("=== End Train ===");
        println!("**************************");
    }
}
