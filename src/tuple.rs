use crate::Runner;
use std::fmt::{Display, Formatter};

pub struct Tuple {}

impl Runner for Tuple {
    fn run(&self) {
        // 長いタプルの定義
        let long_tuple = (
            1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
        );

        // 出力してみる
        println!("long tuple first value: {}", long_tuple.0);
        println!("long tuple second value: {}", long_tuple.1);

        // Tupleの入れ子も可能
        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
        println!("tuple of tuples: {:?}", tuple_of_tuples);

        // 長すぎるとprintできない(Debugがデフォルトで実装されていないため)
        // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
        // println!("too long tuple: {:?}", too_long_tuple);

        let pair = (1, true);
        println!("pair is {:?}", pair);
        println!("the reversed pair is {:?}", reverse(pair));

        // 要素を一つしか持たない場合は括弧で囲まれたただのリテラルと区別するためにカンマが必要
        println!("one element tuple: {:?}", (5u32,));
        println!("just an integer: {:?}", (5u32));

        let tuple = (1, "hello", 4.5, true);

        // バラして代入も可能
        let (a, b, c, d) = tuple;
        println!("{a:?}, {b:?}, {c:?}, {d:?}");

        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{matrix:?}");
    }

    fn train(&self) {
        // Displayを実装した
        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{matrix}");

        // transposeの確認
        println!("Matrix:\n{}", matrix);
        println!("Transpose:\n{}", transpose(matrix));
    }
}

// 以下の感じで返り値などに使える、展開も可能
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {}, {} )\n( {}, {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}
