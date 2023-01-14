use crate::Runner;

pub struct ArraySlice {}

impl Runner for ArraySlice {
    fn run(&self) {
        // 固定長の配列
        let xs: [i32; 5] = [1, 2, 3, 4, 5];

        // 0埋めできる
        let ys: [i32; 500] = [0; 500];

        // 0始まり
        println!("first element of the array: {}", xs[0]);
        println!("second element of the array: {}", xs[1]);

        // lenで要素数を返す
        println!("number of elements in array: {}", xs.len());

        // 配列はスタック上に置かれる
        println!("array occupies {} bytes", std::mem::size_of_val(&xs));

        // 配列は自動的にスライスとして借用される。
        println!("borrow the whole array as a slice");
        analyze_slice(&xs);

        // スライスは配列の一部を指すことができる。
        println!("borrow a section of the array as a slice");
        analyze_slice(&ys[1..4]);

        // インデックスが範囲外のときはコンパイルエラー(this operation will panic at runtime)
        // println!("{}", xs[5]);

        // スライスのアドレス
        println!("array addr is {:p}(start addr)", xs.as_ptr());
        println!("slice addr points head element addr 0 => {:p}", &xs[0]);
        println!("slice addr points head element addr 1 => {:p}", &xs[1]);
        println!("slice addr points head element addr 2 => {:p}", &xs[2]);
    }
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}
