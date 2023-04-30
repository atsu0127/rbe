use crate::Runner;

pub struct VariableBinding {}

impl Runner for VariableBinding {
    fn run(&self) {
        // 可変と不変の変数定義
        let imm = 10;
        let mut mu = 10;
        println!("[before] imm: {}, mu: {}", imm, mu);
        // 以下は成功する
        mu = 15;
        // 以下は失敗する
        // imm = 100;
        println!("[after] imm: {}, mu: {}", imm, mu);

        // 変数のスコープは基本的にはblock
        let long = 100;
        {
            let short = 50;
            println!("short: {}", short);
        }
        // shortはもう使えない
        // println!("short: {}", short);
        // longは使える
        println!("long: {}", long);

        // shadowingも可能
        let shadow = 100;
        let shadow = 150;

        // 先に宣言だけしておくこともできる
        let a_ini;
        {
            let x = 2;
            // こっちで値入れてみる
            a_ini = x * x;
        }
        // a_iniはもちろん参照可能(xが構造体で借用だった場合はダメそう)
        println!("a_ini: {}", a_ini);

        // 宣言だけしたやつの参照はエラーになる
        let b_ini: u32;
        // println!("b_ini: {}", b_ini);

        // 可変の変数も、shadowingで不変の変数にしてしまえばfreezeできる
        let mut _muta = 100;
        {
            let _muta = _muta;
            // このスコープでは_mutaは不変なので、以下は失敗する
            // _muta = 10;
        }
        // スコープを出たので_mutaは可変に戻る
        _muta = 10;
    }
}
