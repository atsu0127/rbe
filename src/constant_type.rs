use crate::Runner;

pub struct ConstantType {}

impl Runner for ConstantType {
    fn run(&self) {
        // constは変更不可な変数
        const THRESHOLD: i32 = 10;
        // staticは'staticなlifetimeの変更されうる変数、この変数へのアクセスはunsafe
        static mut LANGUAGE: &str = "Rust";

        fn is_big(n: i32) -> bool {
            n > THRESHOLD
        }
        let n = 15;

        unsafe {
            println!("This is {}", LANGUAGE);
        }
        println!("The threshold is {}", THRESHOLD);
        println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

        unsafe {
            LANGUAGE = "english";
            println!("This is {}", LANGUAGE);
        }

        // 以下はエラーになる
        //  THRESHOLD = 10;
    }
}
