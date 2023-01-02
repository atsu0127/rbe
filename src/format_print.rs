use crate::Runner;
use std::fmt::{Display, Formatter};

pub struct FormatPrint {}

#[allow(dead_code)]
struct Structure(i32);

impl Runner for FormatPrint {
    fn run(&self) {
        // 単純な例
        println!("{} days", 31);
        // 引数つき
        println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
        // 名前での引数
        println!(
            "{subject} {verb} {object}",
            object = "The lazy dog",
            subject = "the quick brown fox",
            verb = "jumps over"
        );
        // :でのフォーマット指定
        println!(
            "{} of {:b} people lnow binary, the other half doesn't",
            1, 2
        );
        // 空白でパディング
        println!("{number:>width$}", number = 1, width = 6);
        // 0でパディング
        println!("{number:0>width$}", number = 1, width = 6);
        // 以下だと引数足りなくてエラーになる
        // println!("My name is {0}, {1} {0}", "Bond");
        // カスタム構造体の出力(これもエラーになる)
        // println!("This struct `{}` won't print...", Structure(3));
    }

    fn train(&self) {
        // 引数が足りない場合
        println!("My name is {0}, {1} {0}", "Bond", "James");
        // カスタム構造体の場合
        impl Display for Structure {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("Structure({})", self.0))
            }
        }
        println!("This struct `{}` will print!!", Structure(3));
        // Pi is roughly 3.142を出力する
        let pi = 3.141592;
        println!("Pi is roughly {pi:.prec$}", prec = 3);
    }
}
