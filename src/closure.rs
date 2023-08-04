#![allow(clippy::drop_copy)]

use crate::Runner;

pub struct Closure {}

impl Runner for Closure {
    fn run(&self) {
        // closureは|val| val + xという書式になる
        let outer_var = 42;

        // 以下の感じでクロージャをかける
        let closure_annotated = |i: i32| -> i32 { i + outer_var };
        let closure_inferred = |i: i32| i + outer_var;
        println!("closure_annotated: {}", closure_annotated(1));
        println!("closure_inferred: {}", closure_inferred(1));

        // 引数を取らないのもある
        let one = || 1;
        println!("closure returning one: {}", one());

        use std::mem;
        // closureは外部の変数を参照・可変参照・値そのものの三形態で確認できる
        let mut color = String::from("green");

        // これはcolorを借用して、クロージャprintに保持する
        let print = || println!("`color`: {color}");

        // printしてみる
        print();

        // 可変ではない借用は可能
        let _reborrow = &color;
        print();

        // 可変参照はprintをこの後使う場合はできない
        // printがあるので可変参照で以下のエラーになる
        // cannot borrow `color` as mutable because it is also borrowed as immutable
        // let _mut_reborrow = &mut color;
        // print();

        // moveもprintをこの後使う場合はできない
        // cannot move out of `color` because it is borrow
        // let _color_moved = color;
        // print();

        let mut count = 0;
        // countの可変借用している
        let mut inc = || {
            count += 1;
            println!("`count`: {count}");
        };

        // 実行してみる
        inc();

        // またinc使うので再度借用はできない
        // cannot borrow `count` as immutable because it is also borrowed as mutable
        // let _reborrow = &count;
        // inc();

        // もしももう使わないなら再度借用できる
        let _count_reborrow = &mut count;

        // コピーできない
        let movable = Box::new(3);
        let consume = || {
            println!("`movable`: {movable:?}");
            mem::drop(movable);
        };
        consume();
        // movableはdropによって消費されるので2度は使えない
        // use of moved value: `consume`
        // consume();

        // コピーできる
        let copyable = 3;
        let consume = || {
            println!("`copyable`: {copyable:?}");
            mem::drop(copyable);
        };
        consume();
        // copyableはコピーされたものがdropされるので、copyableそのものはmoveしない
        consume();

        // TODO: 続きは「捕捉時の型推論」から
    }
}
