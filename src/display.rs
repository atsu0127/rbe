use crate::Runner;
use std::fmt;
use std::fmt::Formatter;

pub struct Display {}

// Point2DにDisplay実装
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl Runner for Display {
    fn run(&self) {
        // StructureにDisplayを実装
        struct Structure(i32);
        impl fmt::Display for Structure {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        let structure3 = Structure(3);
        println!("Structure has {structure3}");

        // MinMaxにDisplay実装
        #[derive(Debug)]
        struct MinMax(i64, i64);

        impl fmt::Display for MinMax {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "({}, {})", self.0, self.1)
            }
        }

        let minmax = MinMax(0, 14);
        println!("Compare structures:");
        println!("Display: {minmax}");
        println!("Debug: {minmax:?}");

        let big_range = MinMax(-300, 300);
        let small_range = MinMax(-3, 3);

        println!("The big range is {big_range} and the small is {small_range}");

        let point = Point2D { x: 3.3, y: 7.2 };
        println!("Compare points:");
        println!("Display: {point}");
        println!("Debug: {point:?}");

        // 以下は `fmt::Binary` が実装されていないのでエラーになる
        // println!("What dies Point2D look like in binary: {point:b}");
    }

    fn train(&self) {
        // Complex構造体作ってみる
        #[derive(Debug)]
        struct Complex {
            real: f64,
            imag: f64,
        }

        impl fmt::Display for Complex {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{} + {}i", self.real, self.imag)
            }
        }

        let comp = Complex {
            real: 3.3,
            imag: 7.2,
        };

        println!("Display: {comp}");
        println!("Debug: {comp:?}");
    }
}
