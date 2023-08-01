#![allow(overflowing_literals)]
#![allow(clippy::cast_nan_to_int)]

use crate::Runner;

pub struct TypeCast {}

impl Runner for TypeCast {
    fn run(&self) {
        let decimal = 65.4321_f32;
        // 暗黙の型変換はできない
        // let integer: u8 = decimal;

        // やるなら明示的に
        let integer = decimal as u8;
        let character = integer as char;

        // とはいえできない変換もある(小数を文字列に変換はできない)
        // let character = decimal as char;

        println!("Casting: {decimal} -> {integer} -> {character}");

        // キャストの仕様として、ある値を符号無しの型(Tとする)にキャストすると
        // 値がTの範囲に収まるまでTの最大値+1が加減算される
        // u16は0~65535なので1000はおさまっている
        println!("1000 as a u16 is: {}", 1000_u16);
        // u8は0~255なので1000は1000-256-256-256=232になる
        println!("1000 as a u8 is: {}", 1000_u8);
        // 正の数の場合はmodと同じ
        println!("1000 mod 256 is: {}", 1000 % 256);

        // 符号月の型の場合は、以下を行った場合に等しい
        // 1. 対応する符号無しの型にキャスト
        // 2. 2の補数表記として解釈
        // 128をi8で表す場合
        // 1. 128をu8にキャストすると128(0b10000000)
        // 2. 8bitの範囲で2の補数として解釈すると先頭が1なので負の数、残りをbit反転して1足すと0b10000000(128)
        println!("128 as a i8 is : {}", 128_i8);
        // 1000をi8で表す場合
        // 1. 1000はu8では232(0b11101000)
        // 2. 先頭が1なので負の数、残りをbit反転して1足すと0b00011000(24)
        println!("1000 as a i8 is : {}", 1000_i8);

        // 浮動小数点を整数にキャストすると、asは*飽和的キャスト*をする
        // 上限を超えたり下限を下回っている場合は、戻り値は超えられた境界の値になる
        // 1000.0は255
        println!("1000.0 as u8 is : {}", 1000.0_f32 as u8);
        // -100.0は0
        println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
        // nanも0
        println!("nan as u8 is : {}", f32::NAN as u8);
        // 上記は実行時のコストがかかるので、以下で整数として扱うことでコスト回避できる。
        // ただしunsafeでオーバーフローしたり不正確な値を返すので注意
        unsafe {
            // 1000.0 as u8 is 232
            println!("1000.0 as u8 is : {}", 1000.0_f32.to_int_unchecked::<u8>());
            // -100.0 as u8 is 156
            println!(
                "-100.0 as u8 is : {}",
                (-100.0_f32).to_int_unchecked::<u8>()
            );
            // nan as u8 is 0
            println!("nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
        }
    }
}
