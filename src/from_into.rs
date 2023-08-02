use crate::Runner;

pub struct FromIntoStruct {}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

impl Runner for FromIntoStruct {
    fn run(&self) {
        // Fromトレイトはある型に対して別の型からその型を作る方法を定義する
        // str→stringは以下の感じ
        let my_str = "hello";
        let my_string = String::from(my_str);
        println!("my_string: {my_string}");

        // 自作の型でも可能
        let num = Number::from(30);
        println!("My Number is {num:?}");

        // IntoはFromの逆
        let int = 5;
        // だけど型アノテーションがないと何になるかわからないので、このNumberは必須
        let num: Number = int.into();
        println!("My number is {num:?}");
    }
}
