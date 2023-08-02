#![allow(clippy::never_loop)]

use crate::Runner;
use std::str::FromStr;

pub struct Condition {}

impl Runner for Condition {
    fn run(&self) {
        // loopにはラベルがつけられる
        println!("label付きのloop、innerがstartするがfinishは出ないはず");
        'outer: loop {
            println!("start outer");

            'inner: loop {
                println!("start inner");
                break 'outer;
            }

            println!("finish inner");
        }
        println!("finish outer");

        // loopの返り値を変数に入れられる
        let mut counter = 0;
        let loop_res = loop {
            counter += 1;
            if counter < 11 {
                break counter; // breakの引数に返したい値を入れる
            }
        };
        println!("loop result is {loop_res}");

        // forとイテレータについて
        // 配列をイテレータにするのは3つ方法がある
        // iterの場合: コレクションの要素を借用する(= loop抜けた後も使える)
        let names = vec!["Alice", "Bob", "Carl"];
        for name in names.iter() {
            match name {
                &"Bob" => println!("I'm Bob!!"),
                _ => println!("No body"),
            }
        }
        println!("names is {names:?}");

        // into_iterの場合: コレクションの中身をloopにmoveするので再利用できない
        for name in names.into_iter() {
            match name {
                "Bob" => println!("I'm Bob!!"),
                _ => println!("No body"),
            }
        }
        // 以下でエラーになる
        // value borrowed here after move
        // println!("names is {names:?}");

        // iter_mut: 可変参照を取る、借用なので再利用可能
        let mut names = vec!["Alice", "Bob", "Carl"];
        for name in names.iter_mut() {
            *name = match name {
                &mut "Bob" => "I'm Bob!!",
                _ => "No body",
            }
        }
        println!("mutated names is {names:?}");

        // matchの書き方
        // 全通りを網羅しないといけない
        let number = 3;
        println!("Tell me about {number}");
        match number {
            1 => println!("one"),
            2 | 3 | 5 => println!("Prime"),
            13..=19 => println!("13..=19"),
            _ => println!("others"),
        }

        // matchを使ったデストラクト
        // tuple
        let triple = (1, -1, 100);
        println!("destruct tuple => {triple:?}");
        match triple {
            (1, x, y) => println!("最初が0、その他をx: {x}, y: {y}でバインド"),
            (0, ..) => println!("最初が1、あとは気にしない"),
            (.., 2) => println!("最後が2、あとは気にしない"),
            (3, .., 4) => println!("最後が3で最後が4、あとは気にしない"),
            _ => println!("その他"),
        }

        // arrayとslice
        let array = [1, -2, 6, 10];
        println!("destruct array => {array:?}");
        match array {
            [0, second, third, ..] => println!("最初が0であとはsecond: {second}とthird: {third}にバインド"),
            [-1, second, ..] => println!("最初が-1でsecond: {second}にバインド、あとは知らない"),
            [1, second, tail @ ..] => println!("最初が1でsecond: {second}にバインド、また残りをtail: {tail:?}にバインド"),
            [first, middle @ .., last] => println!("first: {first}, middle: {middle:?}, last: {last}みたいに真ん中を配列にバインドもできる"),
        }

        // enum
        #[allow(dead_code)]
        #[derive(Debug)]
        enum Color {
            // These 3 are specified solely by their name.
            // これら3つの値は名前のみで扱うことができる
            Red,
            Blue,
            Green,
            // These likewise tie `u32` tuples to different names: color models.
            // 以下の値は名前と`u32`のタプルをペアにしている。
            // カラーモデルと呼ばれる。
            RGB(u32, u32, u32),
            HSV(u32, u32, u32),
            HSL(u32, u32, u32),
            CMY(u32, u32, u32),
            CMYK(u32, u32, u32, u32),
        }
        let color = Color::RGB(122, 17, 40);
        println!("What color is {color:?}");
        match color {
            Color::Red => println!("The color is Red!"),
            Color::Blue => println!("The color is Blue!"),
            Color::Green => println!("The color is Green!"),
            Color::RGB(r, g, b) => println!("Red: {r}, green: {g}, and blue: {b}!"),
            Color::HSV(h, s, v) => println!("Hue: {h}, saturation: {s}, value: {v}!"),
            Color::HSL(h, s, l) => println!("Hue: {h}, saturation: {s}, lightness: {l}!"),
            Color::CMY(c, m, y) => println!("Cyan: {c}, magenta: {m}, yellow: {y}!"),
            Color::CMYK(c, m, y, k) => {
                println!("Cyan: {c}, magenta: {m}, yellow: {y}, key (black): {k}!",)
            }
        }

        // ポインタとref
        // i32への参照
        let reference = &4;
        println!("deref reference: {reference:p}");
        match reference {
            &val => println!("参照のderef結果: {val:?}"),
        }
        match *reference {
            val => println!("参照のderef(&を使わない)結果: {val:?}"),
        }

        // 普通に値を渡すと参照にはならないので、refをつける(&3と同じ)
        let ref _is_reference = 3;

        // refを使ってmatchもできる(↑と同じ
        let value = 5;
        match value {
            ref r => println!("値へのref取得: {r:?}@{r:p}"),
        }

        // 可変な参照場合はref mutを使う
        let mut mut_value = 6;
        let ref_mut_value = &mut_value;
        println!("10を足す前: {mut_value:?}@{ref_mut_value:p}");
        match mut_value {
            ref mut m => {
                *m += 10;
                println!("10を足しました: {m:?}@{m:p}");
            }
        }

        // 構造体
        struct Foo {
            x: (u32, u32),
            y: u32,
        }
        let foo = Foo { x: (1, 2), y: 3 };
        match foo {
            Foo { x: (1, b), y } => {
                println!("xの一つ目が1で、二つ目がb: {b}、yはy: {y:?}でバインドできている")
            }
            Foo { y: 2, x: i } => println!("yは2でxはi: {i:?}にバインド"), // 順序は関係ない
            Foo { y, .. } => println!("yはy: {y}にバインドして、あとは知らない"),
        }
        let foo = Foo { x: (1, 2), y: 3 };
        // 代入式でのデストラクトも可能
        let Foo { x: x0, y: y0 } = foo;
        println!("x0: {x0:?}, y0: {y0}");

        // match guard
        #[allow(dead_code)]
        enum Temperature {
            Celsius(i32),
            Fahrenheit(i32),
        }

        let temperature = Temperature::Celsius(35);
        match temperature {
            // ガードの使い方
            Temperature::Celsius(y) if y > 30 => println!("{y}Cが30より上です"),
            Temperature::Celsius(y) => println!("{y}Cが30より下です"),
            _ => println!("その他"),
        }

        // match binding
        // 変数 @ <マッチ条件>の形でかけます
        let age = 11u32;
        match age {
            0 => println!("0才です"),
            n @ 1..=12 => println!("1~12才です: {n:?}"),
            _ => println!("その他"),
        }
        // Someとかにも使える
        let some_num = Some(42);
        match some_num {
            Some(n @ 42) => println!("42でした: {n:?}"),
            _ => println!("その他"),
        }

        // if letバインド
        // optionalな値をマッチさせる時にNoneが不要だとmatch分だと冗長になる
        // 以下の感じ
        // match some_num {
        //     Some(i) => {
        //         println!("i is {i:?}");
        //     }
        //     _ => {}
        // }
        // そこでif letでバインドできる
        let number = Some(7);
        let letter: Option<i32> = None;
        let emoticon: Option<i32> = None;
        // 以下の感じ
        if let Some(i) = number {
            println!("Matched {i:?}!");
        }
        // Someにならない場合もelseで拾える
        if let Some(i) = letter {
            println!("Matched {i:?}!");
        } else {
            println!("letter is None!!");
        }
        // デストラクトでSomeじゃなかった時をさらに分岐できる
        let i_like_letter = false;
        if let Some(i) = emoticon {
            println!("Matched {i:?}!");
        } else if i_like_letter {
            println!("i like letter");
        } else {
            println!("emoticon is None and i don't like letter");
        }
        // enumもバインドできる
        enum Fii {
            Bar,
            Baz,
            Qux(u32),
        }
        let a = Fii::Qux(199);
        // Quxだったら絶対ここに入る
        if let Fii::Qux(i) = a {
            println!("a is Qux({i})");
        }
        // Quxの値でもマッチできる
        if let Fii::Qux(value @ 199) = a {
            println!("a is Qux and value is 199: {value}");
        }

        // let else(swiftでいうguard let else)
        // test_strを数値以外の文字列にするとpanicで止まる
        let test_str = "11";
        let Ok(count) = u64::from_str(test_str) else {
            panic!("test_strを数値にできませんでした");
        };
        println!("test_str: {test_str:?}はcount: {count:?}に変換されました");

        // while let
        // loopでmatch文を使う際に、_ => {break;}みたいなのを書かないようにできる
        let mut optional = Some(0);
        while let Some(i) = optional {
            if i > 5 {
                println!("gt 5: {i:?}");
                optional = None;
            } else {
                println!("lt 5: {i:?}");
                optional = Some(i + 1);
            }
        }
    }
}
