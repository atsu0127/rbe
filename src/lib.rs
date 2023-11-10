#![feature(type_name_of_val)]

extern crate core;

use crate::array_slice::ArraySlice;
use crate::closure::Closure;
use crate::condition::Condition;
use crate::constant_type::ConstantType;
use crate::debug::Debug;
use crate::display::Display;
use crate::diverging_function::DivergingFunction;
use crate::enum_type::EnumType;
use crate::format_print::FormatPrint;
use crate::from_into::FromIntoStruct;
use crate::hello_world::HelloWorld;
use crate::hof::Hof;
use crate::literal::Literal;
use crate::literal_operator::LiteralOperator;
use crate::method::Method;
use crate::structure::Structure;
use crate::try_from_into::TryFromIntoStruct;
use crate::tuple::Tuple;
use crate::type_cast::TypeCast;
use crate::variable_binding::VariableBinding;
use std::any::type_name_of_val;

pub mod array_slice;
pub mod closure;
pub mod condition;
pub mod constant_type;
pub mod debug;
pub mod display;
pub mod diverging_function;
pub mod enum_type;
pub mod format_print;
pub mod from_into;
pub mod hello_world;
pub mod hof;
pub mod literal;
pub mod literal_operator;
pub mod method;
pub mod modules;
pub mod structure;
pub mod try_from_into;
pub mod tuple;
pub mod type_cast;
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
        Box::new(TypeCast {}),
        Box::new(Literal {}),
        Box::new(FromIntoStruct {}),
        Box::new(TryFromIntoStruct {}),
        Box::new(Condition {}),
        Box::new(Method {}),
        Box::new(Closure {}),
        Box::new(Hof {}),
        Box::new(DivergingFunction {}),
        Box::new(Modules {}),
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
