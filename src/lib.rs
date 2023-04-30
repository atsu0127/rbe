#![feature(type_name_of_val)]

extern crate core;

use crate::array_slice::ArraySlice;
use crate::constant_type::ConstantType;
use crate::debug::Debug;
use crate::display::Display;
use crate::enum_type::EnumType;
use crate::format_print::FormatPrint;
use crate::hello_world::HelloWorld;
use crate::literal_operator::LiteralOperator;
use crate::structure::Structure;
use crate::tuple::Tuple;
use crate::variable_binding::VariableBinding;
use std::any::type_name_of_val;

pub mod array_slice;
pub mod constant_type;
pub mod debug;
pub mod display;
pub mod enum_type;
pub mod format_print;
pub mod hello_world;
pub mod literal_operator;
pub mod structure;
pub mod tuple;
pub mod variable_binding;

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
        Box::new(Structure {}),
        Box::new(EnumType {}),
        Box::new(ConstantType {}),
        Box::new(VariableBinding {}),
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
