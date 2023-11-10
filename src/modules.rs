use crate::Runner;

pub struct Modules {}

impl Runner for Modules {
    fn run(&self) {
        // moduleの基本
        // - moduleないは基本的にprivateで明示的にpublicにする
        // - moduleがnestしている場合pub(super)で親のみに公開できる
        // - pub(crate)で現在のcrateないでのみアクセスできる
        // - nestしたmoduleで親がprivateだと子がpubでもアクセス制限される
        // - pub(in <path>)の場合指定したpath内でアクセスできる

        // 構造体の場合
        // - フィールドの可視性も定義できる(デフォルトprivate)

        // use宣言
        // - useで外部のmoduleに簡潔にアクセスできる
        // - useした関数は元々ある宣言をshadowingできる

        // superとself
        // - superとselfは普通に使える

        // ファイルの階層構造
        // - 基本的にサブモジュールを作るときは、モジュール名のファイルを作り、それを親で`mod <module名>`する
        // - 孫moduleを定義したい場合は親となるmoduleと同じ名前のディレクトリを作成し、そこにモジュール名のファイルを作る。で子moduleで`mod <module名>`する
        // - modする時にpub modしてないと親moduleから参照できなくなる
    }
}
