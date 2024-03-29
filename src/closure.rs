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

        // クロージャを受け取って実行する関数
        fn apply<F>(f: F)
        where
            F: FnOnce(),
        {
            f();
        }
        // クロージャを引数に取りi32を返す関数
        fn apply_to_3<F>(f: F) -> i32
        where
            F: Fn(i32) -> i32,
        {
            f(3)
        }

        let greeting = "hello";
        let mut farewell = "goodbye".to_owned();

        // greetingは参照を補足する
        // farewellは値を補足する
        let diary = || {
            // 上から順に不変参照・可変参照・値そのもの、が必要になるのでFn→FnMut→FnOnceの順で厳しくなっている
            // 参照を補足しているので、Fn以上が必要
            println!("I said {greeting}.");

            // farewellの値を変えるので、FnMut以上で補足しないといけない
            farewell.push_str("!!!");
            println!("Then I screamed {farewell}.");
            println!("Now I can sleep. zzzzz");

            // farewellの値をmoveするので、FnOnceで補足しないといけない
            mem::drop(farewell);
        };

        apply(diary);

        let double = |x| 2 * x;
        println!("3 doubled: {}", apply_to_3(double));

        // 関数にクロージャを渡す場合
        // 関数内でクロージャを使う場合はGenericでクロージャを受け取る必要がある
        // rustのコンパイラは以下の順序で処理している
        // 1. 無名構造体を作り、そこに外側の変数を入れる
        // 2. Fn or FnMut or FnOnce、のどれかトレイトを介してこの構造体に関数として実装する
        // 3. 無名構造体は型がunknownなので、関数実行時にGenericが必要だがselfの引数としての取り方がわからない(&self or &mut self or self)
        // 4. そこでFn or FnMut or FnOnceが必要になる
        fn apply2<F>(f: F)
        where
            F: FnOnce(),
        {
            f()
        }
        let x = 7;
        let print = || println!("{x}");
        apply2(print);

        // 関数に関数を渡す場合
        fn call_me<F: Fn()>(f: F) {
            f();
        }

        fn function() {
            println!("I'm a function!");
        }

        let closure = || println!("I'm a closure!");

        call_me(closure);
        call_me(function);

        // クロージャを返す関数
        // クロージャを返すためには以下が必要
        // 1. impl Traitを使って型を定義する
        // 2. クロージャに参照などの所有権を移すため、moveをクロージャにつける
        fn create_fn() -> impl Fn() {
            let text = "Fn".to_owned();
            move || println!("this is a: {text}")
        }
        fn create_fnmut() -> impl FnMut() {
            let text = "FnMut".to_owned();
            move || println!("This is a: {text}")
        }
        fn create_fnonce() -> impl FnOnce() {
            let text = "FnOnce".to_owned();
            move || println!("This is a: {text}")
        }

        let fn_plain = create_fn();
        let mut fn_mut = create_fnmut();
        let fn_once = create_fnonce();

        fn_plain();
        fn_mut();
        fn_once();

        // 実際にstdでクロージャを使っているIterator::anyを使ってみる
        // anyでは引数にFnMutのクロージャを受けているので、anyを実行した後でも再利用は可能
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![4, 5, 6];

        println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
        println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

        // vec1はiterなので所有権は失ってないので使える
        println!("vec1 len: {}", vec1.len());
        println!("first element of vec1: {}", vec1[0]);

        // vec2はinto_iterなので所有権がmoveしているので以下は失敗する
        // borrow of moved value: `vec2`
        // println!("vec2 len: {}", vec2.len());
        // println!("first element of vec2: {}", vec2[0]);

        // Iterator::findをつかってみる
        // findも引数にFnMutを取っているので変更の可能性はあるけど消費はされない
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![4, 5, 6];

        println!("find 2 in vec1: {:?}", vec1.iter().find(|&&x| x == 2));
        println!("find 2 in vec2: {:?}", vec2.into_iter().find(|&x| x == 2));

        // findは要素への参照を返すけど、インデックスが欲しい場合はpositionを使う
        let vec = vec![1, 9, 3, 3, 13, 2];
        let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
        assert_eq!(index_of_first_even_number, Some(5));
        let index_of_first_even_number = vec.iter().position(|&x| x < 0);
        assert_eq!(index_of_first_even_number, None);
    }
}
